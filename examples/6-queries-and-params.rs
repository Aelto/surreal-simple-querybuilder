#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

use serde_json::json;
use surreal_simple_querybuilder::prelude::*;

// a fake query function, imagine it calls to the DB client of your choices and
// returns what you expect
fn fake_query<T>(query: String, params: HashMap<String, serde_json::Value>) -> T {
  todo!()
}

model!(User {
  id,
  pub age,
  pub name,
});

// 👇 let's imagine we are building a type for our User nodes
struct User {
  id: Option<String>,
  age: u8,
  name: String,
}

impl User {
  fn new(name: String, age: u8) -> Self {
    Self {
      id: None,
      name,
      age,
    }
  }

  // 👇 then we decide to create a function to fetch a user with a specific name:
  // NOTE: errors are ignored for brevity
  fn find_by_name(searched_name: &str) -> Option<Self> {
    use schema::model as user;

    // 👇 we construct our parameters using the `Where` type and we pass it to
    // the select function. It generates the SQL query for us and also gives
    // us a hashmap of the variables that were used in the query and their
    // values so we can pass them to our client.
    let params = Where(json!({ user.name: searched_name }));
    let (query, params) = select("*", &user, params).unwrap();

    fake_query(query, params)
  }

  // 👇 now let's say we'd want to fetch user but with different filters and
  // ideally pagination to avoid fetching thousands of users at once.
  // Coding a method for all types of combination we want (Pagination, Where, Fetch)
  // can quickly become repetitive and cumbersome, let's make a generic function
  // instead.
  //
  // This is how you'd connect the Surreal Simple Querybuilder crate to the
  // client of your choice.
  //
  // Read the `main` function below to see how to use it now
  fn find<'a, T>(params: impl QueryBuilderInjecter<'a> + 'a) -> T {
    use schema::model as user;

    let (query, params) = select("*", &user, params).unwrap();

    fake_query(query, params)
  }
}

fn main() -> Result<(), SqlSerializeError> {
  use schema::model as user;

  // 👇 this will select all users named John.
  //
  //    SELECT * FROM User where name = "John"
  let _all_johns: Vec<User> = User::find(Where(json!({ user.name: "John" })));

  // 👇 this will select all users, but only from the 10th to the 35th one
  //
  //    SELECT * FROM User LIMIT 25 START AT 10
  let _paginated_users: Vec<User> = User::find(Pagination::from(10..35));

  // 👇 the combination of both params from above
  //
  //    SELECT * FROM User WHERE name = "John" LIMIT 25 START AT 10
  let filter = Where(json!({ user.name: "John" }));
  let pagination = Pagination::from(10..35);
  //                                          👇 you can combine them with tuples
  let _paginated_johns: Vec<User> = User::find((filter, pagination));

  Ok(())
}
