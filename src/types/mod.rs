//! "Injecters" or "dynamic parameters" are types that allow you to build queries
//! in a fully composable manner via tuples. Each SQL clause or statement has its
//! injecter type that you can return or pass around as parameters in your functions.
//!
//! ```
//! use surreal_simple_querybuilder::prelude::*;
//!
//! assert_eq!(
//!   query(&(Select("*"), From("users"), Where("id = 5"))).unwrap(),
//!   "SELECT * FROM users WHERE id = 5"
//! );
//! ```
//!
//! Another strength of the injecters is their ability to turn the parts that are
//! usually sensitive to XSS injections. For example:
//! ```
//! use surreal_simple_querybuilder::prelude::*;
//! use serde_json::json;
//!
//! let injecters = (
//!   Select("*"),
//!   From("users"),
//!   Where(And((
//!     ("id", 5),
//!     ("name", "john")
//!   )))
//! );
//!
//! let query_string = query(&injecters).unwrap();
//! let query_bindings = bindings(injecters).unwrap();
//!
//! assert_eq!(query_string, "SELECT * FROM users WHERE id = $id AND name = $name");
//! assert_eq!(query_bindings.get("id"), Some(&json!(5)));
//! assert_eq!(query_bindings.get("name"), Some(&json!("john")));
//! ```
//!
//! ---
//!
//! The first scenario that comes to mind is a standard function to retrieve books
//! by the author:
//! ```rust
//! impl Book {
//!   fn find_by_author_id(id: &str) -> Vec<Self> {
//!     // ...
//!  }
//!}
//!```
//!
//! In some cases you'll need the list of books and nothing else, another time you'll need
//! the results to be paginated, and sometimes you'll want to fetch the author data
//! on top of the books. Considering you may also want to have the books with both pagination
//! and fetch this could potentially result in at least 4 different functions & queries
//! to write.
//!
//! With the dynamic parameters you can update your `find` function to accept optional
//! parameters so that only 1 simple function is needed:
//!
//! ```rust
//! use serde_json::json;
//!
//! impl Book {
//!   fn find_by_author_id<'a>(id: &str, params: impl QueryBuilderInjecter<'a> + 'a) -> Vec<Self> {
//!     let filter = Where(json!({"author": id}));
//!     let combined_params = (filter, params);
//!
//!     let (query, params) = select("*", "Book", combined_params).unwrap();
//!
//!     DB.query(query)
//!       .bind(params)
//!       .await.unwrap()
//!       .get(..).unwrap()
//!  }
//! }
//! ```
//! So you can now do:
//! ```rust
//! let books = Book::find_by_author_id("User:john", ());
//! let paginated_books = Book::find_by_author_id("User:john", Pagination(0..25));
//! let paginated_books_with_author_data = Book::find_by_author_id(
//!   "User:john",
//!   (
//!     Pagination(0..25),
//!     Fetch(["author"])
//!   )
//! );
//! ```
//!
//! You can read more about the injecters in the crates README and its guided examples.
#[test]
fn test_injecters_doccomment() {
  assert_eq!(
    crate::queries::query(&(Select("*"), From("users"), Where("id = 5"))).unwrap(),
    "SELECT * FROM users WHERE id = 5"
  );

  use serde_json::json;

  let injecters = (
    Select("*"),
    From("users"),
    Where(And((("id", 5), ("name", "john")))),
  );

  let query_string = crate::queries::query(&injecters).unwrap();
  let query_bindings = crate::queries::bindings(injecters).unwrap();

  assert_eq!(
    query_string,
    "SELECT * FROM users WHERE id = $id AND name = $name"
  );
  assert_eq!(query_bindings.get("id"), Some(&json!(5)));
  assert_eq!(query_bindings.get("name"), Some(&json!("john")));
}

mod also;
mod and;
mod bind;
mod build;
mod cmp;
mod create;
mod delete;
mod equal;
mod ext;
mod fetch;
mod filter;
mod from;
mod greater;
mod limit;
mod lower;
mod or;
mod order_by;
mod pagination;
mod plus_equal;
mod select;
mod set;
mod sql;
mod update;

pub use also::Also;
pub use and::And;
pub use bind::Bind;
pub use build::Build;
pub use cmp::Cmp;
pub use create::Create;
pub use delete::Delete;
pub use equal::Equal;
pub use ext::*;
pub use fetch::Fetch;
pub use filter::Where;
pub use from::From;
pub use greater::Greater;
pub use limit::Limit;
pub use lower::Lower;
pub use or::Or;
pub use order_by::OrderAsc;
pub use order_by::OrderBy;
pub use order_by::OrderDesc;
pub use pagination::Pagination;
pub use plus_equal::PlusEqual;
pub use select::Select;
pub use set::Set;
pub use sql::Sql;
pub use update::Update;

mod on;
#[cfg(feature = "sql_standard")]
pub use on::On;

pub(crate) fn to_param_value(value: serde_json::Value) -> serde_json::Result<serde_json::Value> {
  Ok(value)
}

/// An internal function made public so the macro generate code can use it. Allows
/// the macro code to use serde_json functions without the parent crates to import
/// the serde_json crate directly.
pub fn ser_to_param_value<T: serde::Serialize>(value: T) -> serde_json::Result<serde_json::Value> {
  to_param_value(serde_json::to_value(value)?)
}
