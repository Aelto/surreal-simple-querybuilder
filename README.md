# Surreal simple querybuilder
A simple query-builder for the Surreal Query Language, for [SurrealDB](https://surrealdb.com/).
Aims at being simple to use and not too verbose first.

# Summary
- [Surreal simple querybuilder](#surreal-simple-querybuilder)
- [Summary](#summary)
- [Why a query-builder](#why-a-query-builder)
- [SQL injections](#sql-injections)
- [Compiler requirements/features](#compiler-requirementsfeatures)
- [Examples](#examples)
  - [Premade queries with dynamic parameters (`queries` feature)](#premade-queries-with-dynamic-parameters-queries-feature)
    - [Why dynamic parameters](#why-dynamic-parameters)
    - [Limitations \& recommandations for premade queries \& params](#limitations--recommandations-for-premade-queries--params)
  - [The `model` macro (`model` feature)](#the-model-macro-model-feature)
    - [public \& private fields in models](#public--private-fields-in-models)
    - [Relations between your models](#relations-between-your-models)
    - [Partials builder generation](#partials-builder-generation)
  - [The `NodeBuilder` traits (`querybuilder` feature)](#the-nodebuilder-traits-querybuilder-feature)
  - [The `QueryBuilder` type (`querybuilder` feature)](#the-querybuilder-type-querybuilder-feature)
  - [The `ForeignKey` and `Foreign` types (`foreign` feature)](#the-foreignkey-and-foreign-types-foreign-feature)
    - [`ForeignKey` and loaded data during serialization](#foreignkey-and-loaded-data-during-serialization)
  - [Using the querybuilder in combination of the official SurrealDB client](#using-the-querybuilder-in-combination-of-the-official-surrealdb-client)

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

# Compiler requirements/features
The crate uses const expressions for its [model creation macros](#the-model-macro)
in order to use stack based arrays with sizes deduced by the compiler. For this reason
any program using the crate has to add the following at the root of the main file:
```
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
```

# Examples

> Keep in mind all of the demonstrated features can be used independently of the
> rest. They can all be combined if you want to, but if you prefer a lightweight
> solution then it is possible as well.
>
> By default only the querybuilder is available, other modules require you to
> enable their respective crate features.

 - A series of [examples are available](/examples/) to offer a **guided introduction** to the core features of the crate
 - An all-in-one example can be found in the alternate [surrealdb-architecture](https://github.com/Aelto/surrealdb-architecture) repository
 - For an explanation of what each component in the crate does, refer to the chapters below.

## Premade queries with dynamic parameters (`queries` feature)
The crate offers a set of premade queries you can access in [`surreal_simple_querybuilder::queries::*;`](src/queries) or
in the prelude for easier access.
```rust
use surreal_simple_querybuilder::prelude::*;

fn main() {
  let (query, _bindings) = select("*", "user", ());

  assert_eq!(query, "SELECT * FROM user");
}
```

these pre-made query functions accept all types of parameters to further extend
the queries. If dynamic values (variables) are passed among these parameters then
the functions will automatically add them to the list of bindings:
```rust
use surreal_simple_querybuilder::prelude::*;
use serde_json::json;

fn main() {
  let (query, bindings) = select("*", "user", Where(json!({ "name": "John" })));

  assert_eq!(query, "SELECT * FROM user WHERE name = $name");

  // 👇 the bindings were updated with the $name variable
  assert_eq!(bindings.get("name"), Some("John".to_owned())); 
}
```

---
### Why dynamic parameters

At a first glance these pre-made queries offer nothing the querybuilder doesn't,
but in reality they allow you to easily make functions in your backends (for example)
that you can extend if need be.

The first scenario that comes to mind is a standard function to retrieve books
by the author:
```rust
impl Book {
  fn find_by_author_id(id: &str) -> Vec<Self> {
    // ...
  }
}
```

In some cases you'll need the list of books and nothing else, another time you'll need
the results to be paginated, and sometimes you'll want to fetch the author data
on top of the books. Considering you may also want to have the books with both pagination
and fetch this could potentially result in at least 4 different functions & queries
to write.

With the dynamic parameters you can update your `find` function to accept optional
parameters so that only 1 simple function is needed:

```rust
use serde_json::json;

impl Book {
  fn find_by_author_id<'a>(id: &str, params: impl QueryBuilderInjecter<'a> + 'a) -> Vec<Self> {
    let filter = Where(json!({"author": id}));
    let combined_params = (filter, params);

    let (query, params) = select("*", "Book", combined_params).unwrap();

    DB.query(query)
      .bind(params)
      .await.unwrap()
      .get(..).unwrap()
  }
}
```
So you can now do:
```rust
let books = Book::find_by_author_id("User:john", ());
let paginated_books = Book::find_by_author_id("User:john", Pagination(0..25));
let paginated_books_with_author_data = Book::find_by_author_id(
  "User:john",
  (
    Pagination(0..25),
    Fetch(["author"])
  )
);
```

The dynamic parameters & premade queries are made with the [model](#the-model-macro) macro in mind,
you don't necessarily need it but if you wanted, both systems can be used for some
compile time checks + dynamic parameters to enjoy the extra freedom dynamic parameters
provide while being sure all of the fields & nodes you reference in them are valid
thanks to the models. A complete example on how to combine both system is [available here](examples/6-queries-and-params.rs).

Alternatively if the use of a generic argument is not your cup of tea, you can use an enum that implements
`QueryBuilderInjecter`. The [surrealdb-architecture](https://github.com/Aelto/surrealdb-architecture#models-queries--params)
repository demonstrates how to setup one.

### Limitations & recommandations for premade queries & params
The [short example](examples/6-queries-and-params.rs) and [complete test case](test/../tests/src/surrealdb_client.rs) demonstrate
the premade queries can work in 99% of the cases and can seriously simplify the
code you write. However there are limitations one must be aware of before going
too deep into the premade queries.

The premade queries and composable parameters are made for those simple cases where
you just want to select/create/etc... elements without complex filtering in the WHERE
clause or anything. For example selecting books by one of their field is perfect
for the premade queries as you can add a fetch clause without having to rewrite
anything. It allows you to have somewhat generic functions in your codebase for the simple cases.

But as soon as it gets complex, the [`QueryBuilder`](src/querybuilder.rs) type should
be used instead of the pre-made queries. It will offer both better performances & more predictable results (nesting lots of params may yield unexpected queries). Note that you can still use a query-builder and pass it params (aka injecters)
if needed:
```rust
use surreal_simple_querybuilder::prelude::*;

let params = (
  Where(("name", "john")),
  Fetch(["articles"])
);

let query = QueryBuilder::new()
  .select("*")
  .from("user")
  .injecter(&params) // <-- pass the injecter to the builder
  .build();

let _params = bindings(params); // <-- get the variables so you can bind them

assert(query, "SELECT * FROM user WHERE name = $name FETCH articles");
```

And as you can see, even in the more complex cases the params can still be used but the pre-made queries should not however.

## The `model` macro (`model` feature)
The `model` macro allows you to quickly create structs (aka models) with fields
that match the nodes of your database.

<details>
  <summary>example</summary>

  ```rust
  use surreal_simple_querybuilder::prelude::*;

  struct Account {
    id: Option<String>,
    handle: String,
    password: String,
    email: String,
    friends: Foreign<Vec<Account>>
  }

  model!(Account {
    id,
    handle,
    password,
    friends<Vec<Account>>
  });

  fn main() {
    // the schema module is created by the macro
    use schema::model as account;

    let query = format!("select {} from {account}", account.handle);
    assert_eq!("select handle from Account", query);
  }
  ```
</details>


This allows you to have compile time checked constants for your fields, allowing
you to reference them while building your queries without fearing of making a typo
or using a field you renamed long time ago.

### public & private fields in models

The QueryBuilder type offers a series of methods to quickly list the fields of your
models in SET or UPDATE statements so you don't have to write the fields and the
variable names one by one. Since you may not want to serialize some of the fields
like the `id` for example the model macro has the `pub` keyword to mark a field
as serializable. Any field without the `pub` keyword in front of it will not
be serialized by these methods.

```rust
model!(Project {
  id, // <- won't be serialized
  pub name, // <- will be serialized
})

fn example() {
  use schema::model as project;

  let query = QueryBuilder::new()
    .set_model(project)
    .build();

  assert_eq!(query, "SET name = $name");
}
```


### Relations between your models
If you wish to include relations (aka edges) in your models, the `model` macro
has a special syntax for them:

```rust
mod account {
  use surreal_simple_querybuilder::prelude::*;
  use super::project::schema::Project;

  model!(Account {
    id,

    ->manage->Project as managed_projects
  });
}

mod project {
  use surreal_simple_querybuilder::prelude::*;
  use super::project::schema::Project;

  model!(Project {
    id,
    name,

    <-manage<-Account as authors
  });
}

fn main() {
    use account::schema::model as account;

    let query = format!("select {} from {account}", account.managed_projects);
    assert_eq!("select ->manage->Project from Account");

    let query = format!("select {} from {account}", account.managed_projects().name.as_alias("project_names"))
    assert_eq!("select ->manage->Project.name as project_names from Account", query);
  }
```

### Partials builder generation
The macro supports condition flags you can pass to generate more code for you. One
of them is the generation of a "Partial" builder. A partial type is a copy of the
model you created where all fields are `Option<serde_json::Value>` set with the serde
flag to skip the fields that are `None` during serialization.

Such a partial builder can be used like so:
```rust
// notice the `with(partial)`
model!(Project with(partial) {
  id,
  pub name
});

let partial_user = PartialProject::new()
  .name("John Doe");
```

This partial type comes handy when constructing queries with nested fields thanks
to its `ok()` method:
```rust
let partial_post = PartialPost::new()
  .title("My post title")
  .author(PartialUser::new().name("John Doe"))
  .ok()?;
```
which will output the following flattened json:
```json
{
  "title": "My post title",
  "author.name": "John Doe"
}
```
If you'd like a normal nested object then you can skip the `ok` call and past the object to the serialize function of your choice.

You can then use the builder in your queries:
```rust
let user = DB.update(user_id)
  .merge(PartialUser::new()
    .name("Jean")
    .posts(vec![post1_id, post2_id])
    .ok()?
  ).await?

// ...

let filter = Where(PartialPost::new()
  .title("My post title")
  .author(PartialUser::new().name("John Doe"))
  .ok()?);

let posts = queries.select("*", "post", filter).await?;
```

Note that partial builders are an alternative syntax to building the json objects using the `serde_json::json!` macro combined with the models. The above example is the same as the following example, so pick whatever solution you prefer:
```rust
let user = DB.update(user_id)
  .merge(json!({
    model.name: "Jean",
    model.posts: vec![post1_id, post2_id]
  })).await?

// ...

// the wjson! macro is a shortcut to `Where(json!())`
let filter = wjson!({
  model.title: "My post title",
  model.author().name: "John Doe"
});

let posts = select("*", "post", filter).await?;
```

## The `NodeBuilder` traits (`querybuilder` feature)
These traits add a few utility functions to the `String` and `str` types that can
be used alongside the querybuilder for even more flexibility.

```rust
use surreal_simple_querybuilder::prelude::*;

let my_label = "John".as_named_label("Account");
assert_eq!("Account:John", &my_label);

let my_relation = my_label
  .with("FRIEND")
  .with("Mark".as_named_label("Account"));

assert_eq!("Account:John->FRIEND->Account:Mark", my_relation);
```


## The `QueryBuilder` type (`querybuilder` feature)
It allows you to dynamically build complex or simple queries out of _segments_ and easy to use
methods.
<details>
  <summary>Simple example</summary>

  ```rust
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

  ```rust
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


## The `ForeignKey` and `Foreign` types (`foreign` feature)
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

  ```rust
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
    fn into_key<E>(&self) -> Result<String, E>
    where
      E: serde::ser::Error,
    {
      self
        .id
        .as_ref()
        .map(String::clone)
        .ok_or(serde::ser::Error::custom("The account has no ID"))
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

### `ForeignKey` and loaded data during serialization

A `ForeignKey` always tries to serialize itself into an ID by default. Meaning that
if the foreign-key holds a value and not an ID, it will call the `IntoKey` trait on
the value in order to get an ID to serialize.

There are cases where this may pose a problem, for example in an API where you wish
to serialize a struct with `ForeignKey` fields so the users can get all the data
they need in a single request.

By default if you were to serialize a `File` (from the example above) struct
with a fetched `author`, it would automatically be converted into the author's id.

The `ForeignKey` struct offers two methods to control this behaviour:
```rust
// ...imagine `query` is a function to send a query and get the first result...
let file: File = query("SELECT * from File FETCH author");

file.author.allow_value_serialize();

// ... serializing `file` will now serialize its author field as-is.

// to go back to the default behaviour
file.author.disallow_value_serialize();
```

You may note that mutability is not needed, the methods use interior mutability
to work even on immutable ForeignKeys if needed.

## Using the querybuilder in combination of the [official SurrealDB client](https://github.com/surrealdb/surrealdb/tree/main/lib)
There is an important thing to keep in mind with this querybuilding crate, it is meant to serve as an utility crate that is completely independant of the client you use. For this reason it does not offer anything to send the queries and getting the responses directly but since you'll rarely want to use this crate without a client, I am maintaining an [external repository as a demo of how to combine the official client & the surreal-simple-querybuilder crate](https://github.com/Aelto/surrealdb-architecture).

While it is not convenient to have to write these functions yourself it allows you to use a fixed version of the querybuilder crate while still getting the latest breaking updates on your favorite client.

