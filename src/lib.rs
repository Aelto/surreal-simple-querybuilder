#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

/// Macro utility to easily declare the various fields of a model for use while
/// building queries.
///
/// # Example
/// ```
/// #![feature(generic_const_exprs)]
///
/// // making modules for the example, but it is not needed.
/// mod user {
///   // the macros generate code that need all of the types of the `model` module
///   // so it is shorter to fully import it.
///   use surreal_simple_querybuilder::model;
///
///   use super::file::schema::File;
///   
///   model!(User {
///     id,
///     handle,
///
///     files<File>
///   });
/// }
///
/// mod file {
///   use surreal_simple_querybuilder::model;

///
///   use super::user::schema::User;
///
///   model!(File {
///     id,
///     name,
///
///     // this is how a field that references a foreign type is declared. Allowing
///     // you to access the User's fields easily even if starting from the File
///     // type itself.
///     author<User>
///   });
/// }
///
/// fn main() {
///   // the macro generates a `model` module with two elements in it:
///   //  - a `File` struct
///   //  - a `model` constant whose type is `File` so you can use it directly
///   //    without instantiating a new File each time.
///   use file::schema::model as file;
///
///   // you are now able to use the file constant and any of its fields as it they
///   // were strings, but in a safer manner as it is checked at compile time
///   // if the fields exist.
///   //
///   // note that `to_string()` is used here, but it is not needed when the fields
///   // are passed to the QueryBuilder
///   assert_eq!("author", file.author.to_string());
///
///   // accessing one of the Author's field from the file type
///   assert_eq!("author.handle", file.author().handle.to_string());
///
///   // supports cyclical references
///   assert_eq!("author.files.author.files.name", file.author().files().author().files().name.to_string());
///
///   // the ToNodeBuilder trait is also implemented for the models
///   use surreal_simple_querybuilder::node_builder::ToNodeBuilder;
///
///   assert_eq!("author.files = $author_files", file.author().files.equals_parameterized());
///
///   // if the type is passed rather than a field, then the type is written instead
///   assert_eq!("User", file.author().to_string());
///
///   use user::schema::model as user;
///   assert_eq!("User:john", "john".as_named_label(&user.to_string()));
///
///   // using `format!` rather than the QueryBuilder to keep it simple:
///   let query = format!(
///     "select {} from {file} as TheFile",
///     file.author().handle.from_alias("TheFile")
///   );
///
///   assert_eq!("select TheFile.author.handle from File as TheFile", query);
/// }
/// ```
pub mod model;

/// Contains a trait for simplifying the building of relationships between nodes
pub mod node_builder;

/// Contains the query builder for simplifying the building of Surreal QL queries.
/// Particularely useful when composing variables and conditional queries
pub mod querybuilder;

/// Contains the `Foreign<T>` type used to represent fields that may or may not be
/// loaded.
// pub mod foreign;
pub mod foreign_key;

pub mod prelude;
