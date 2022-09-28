# Surreal simple querybuilder
A simple querybuilder for the Surreal Query Language, for [SurrealDB](https://surrealdb.com/).
Aims at being simple to use and not too verbose first.

# Why a query-builder
Query builders allow you to dynamically build your queries with some compile time
checks to ensure they result in valid SQL queries. Unlike ORMs, query-builders are
built to be lightweight and easy to use, meaning you decide when and where to use
one. You could stick to hard coded string for the simple queries but use a builder
for complex ones that require parameters & variables and may change based on these
variables for example.

While the crate is first meant as a query-building utility, it also comes with
macros and generic types that may help you while managing you SQL models in your rust code.
Refer to the [node macro](#the-node-macro) and the [Foreign type](#the-foreignkey-and-foreign-type) example

# SQL injections
The strings you pass to the query builder are not sanitized in any way. Please use
parameters in your queries like `SET username = $username` with surrealdb parameters to avoid injection issues.
However the crate comes with utility functions to easily create parameterized fields, refer to the [`NodeBuilder`](src/node_builder.rs) trait.

# Examples
A complete example can be found in the [`test.rs`](./src/test.rs) file. For an explanation of each component of the crate refer to the chapters below.
## The `node` macro
The `node` macro allows you to quickly create constants that match the fields of
your structs. It is not a derive macro to allow you to name the fields the way
you want.

<details>
  <summary>example</summary>

  ```rs
  use surreal_simple_querybuilder::prelude::*;

  struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
  }

  node!(Account {
    id,
    handle,
    password,
  });

  fn main() {
    // you can now reference the fields like so:
    let handle_field = schema::handle;
    let password_field = schema::password;

    use schema::*;

    format!("There is also the {id} field");
  }
  ```
</details>


This allows you to have compile time checked constants for your fields, allowing
you to reference them while building your queries without fearing of making a typo
or using a field you renamed long time ago.


## The `NodeBuilder` traits
These traits add a few utility functions to the `String` and `str` types that can
be used alongside the querybuilder for even more flexibility.

```rs
use surreal_simple_querybuilder::prelude::*;

let my_label = "John".as_named_label("Account");
assert_eq!("Account:John", &my_label);

let my_relation = my_label
  .with("FRIEND")
  .with("Mark".as_named_label("Account"));

assert_eq!("Account:John->FRIEND->Account:Mark", my_relation);
```


## The `QueryBuilder` type
It allows you to dynamically build complex or simple queries out of _segments_ and easy to use
methods.
<details>
  <summary>Simple example</summary>

  ```rs
  use surreal_simple_querybuilder::prelude::*;

  let query = QueryBuilder::new()
    .select("*")
    .from("Account")
    .build();

  assert_eq!("SELECT * FROM Account", &query);
  ```
</details>

<details>
  <summary>Complex example</summary>

  ```rs
  use surreal_simple_querybuilder::prelude::*;

  let should_fetch_authors = false;
  let query = QueryBuilder::new()
    .select("*")
    .from("File")
    .if_then(should_fetch_authors, |q| q.fetch("author"))
    .build();

  assert_eq!("SELECT * FROM Account", &query);

  let should_fetch_authors = true;
  let query = QueryBuilder::new()
    .select("*")
    .from("File")
    .if_then(should_fetch_authors, |q| q.fetch("author"))
    .build();

  assert_eq!("SELECT * FROM Account FETCH author", &query);
  ```
</details>


## The `ForeignKey` and `Foreign` type
SurrealDB has the ability to fetch the data out of foreign keys. For example:
```sql
create Author:JussiAdlerOlsen set name = "Jussi Adler-Olsen";
create File set name = "Journal 64", author = Author:JussiAdlerOlsen;

select * from File;
select * from File fetch author;
```
which gives us
```json
// without FETCH author
{
  "author": "Author:JussiAdlerOlsen",
  "id":"File:rg30uybsmrhsf7o6guvi",
  "name":"Journal 64"
}

// with FETCH author
{
  "author": {
    "id":"Author:JussiAdlerOlsen",
    "name":"Jussi Adler-Olsen"
  },
  "id":"File:rg30uybsmrhsf7o6guvi",
  "name":"Journal 64"
}
```

The "issue" with this functionality is that our results may either contain an ID
to the author, no value, or the fully fetched author with its data depending on
the query and whether it includes `fetch` or not.

The `ForeignKey` types comes to the rescue. It is an enum with 3 variants:
 - The loaded data for when it was fetched
 - The key data for when it was just an ID
 - The unloaded data when it was null (if you wish to support missing data you must use the `#serde(default)` attribute to the field)

The type comes with an implementation of the Deserialize and Serialize serde traits
so that it can fallback to whatever data it finds or needs. However any type that
is referenced by a `ForeignKey` must implement the `IntoKey` trait that allows it
to safely serialize it into an ID during serialization.

<details>
  <summary>example</summary>

  ```rs
  /// For the tests, and as an example we are creating what could be an Account in
  /// a simple database.
  #[derive(Debug, Serialize, Deserialize, Default)]
  struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
  }

  impl IntoKey<String> for Account {
    fn into_key(&self) -> String {
      // the unwrap here will cause our program to crash if we were to attempt a
      // serialization of an object without an ID. Keeping it simple for the example
      self.id.as_ref().map(String::clone).unwrap()
    }
  }

  #[derive(Debug, Serialize, Deserialize)]
  struct File {
    name: String,

    /// And now we can set the field as a Foreign node
    author: Foreign<Account>,
  }

  fn main() {
    // ...imagine `query` is a function to send a query and get the first result...
    let file: File = query("SELECT * from File FETCH author");

    if let Some(user) = file.author.value() {
      // the file had an author and it was loaded
      dbg!(&user);
    }

    // now we could also support cases where we do not want to fetch the authors
    // for performance reasons...
    let file: File = query("SELECT * from File");

    if let Some(user_id) = file.author.key() {
      // the file had an author ID, but it wasn't fetched
      dbg!(&user_id);
    }

    // we can also handle the cases where the field was missing
    if file.author.is_unloaded {
      panic!("Author missing in file {file}");
    }
  }
  ```
</details>
