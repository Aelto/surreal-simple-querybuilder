use std::str::FromStr;

use lalrpop_util::lalrpop_mod;
use proc_macro::TokenStream;

mod ast;

lalrpop_mod!(parser);

/// The `model` macro allows you to quickly create structs (aka models) with fields
/// that match the nodes of your database.
///
/// ```rust
/// use surreal_simple_querybuilder::prelude::*;

/// struct Account {
///   id: Option<String>,
///   handle: String,
///   password: String,
///   email: String,
///   friends: Foreign<Vec<Account>>
/// }
///
/// model!(Account {
///   id,
///   handle,
///   password,
///   friends<Vec<Account>>
/// });
///
/// fn main() {
///   the schema module is created by the macro
///   use schema::model as account;
///
///   let query = format!("select {} from {account}", account.handle);
///   assert_eq!("select handle from Account", query);
/// }
/// ```
///
/// This allows you to have compile time checked constants for your fields, allowing
/// you to reference them while building your queries without fearing of making a typo
/// or using a field you renamed long time ago.
///
/// If you wish to include relations (aka edges) in your models, the `model` macro
/// has a special syntax for them:
///
/// ```rust
/// mod account {
///   use surreal_simple_querybuilder::prelude::*;
///   use super::project::schema::Project;
///
///   model!(Account {
///     id,
///
///     ->manage->Project as managed_projects
///   });
/// }
///
/// mod project {
///   use surreal_simple_querybuilder::prelude::*;
///   use super::project::schema::Project;
///
///   model!(Project {
///     id,
///     name,
///
///     <-manage<-Account as authors
///   });
/// }
///
/// fn main() {
///   use account::schema::model as account;
///
///   let query = format!("select {} from {account}", account.managed_projects);
///   assert_eq!("select ->manage->Project from Account");
///
///   let query = format!("select {} from {account}", account.managed_projects().name.as_alias("project_names"))
///   assert_eq!("select ->manage->Project.name as project_names from Account", query);
/// }
/// ```
///
/// The macro automatically creates a module named `schema` with two main elements
/// inside:
///  - a struct named the same way as your model
///  - a `model` constant that is an instance of the struct above so you can quickly
/// use it without having to call `Account::new()` everytime.
///
/// Here is an trimmed down version of what to expect, keep in mind this is an example
/// and not exactly what you will find:
/// ```rs
/// mod schema {
///   struct Account {
///     id: &'static str
///     name: &'statis str,
///     // ...
///   }
///
///   pub const model: Account = Account::new();
/// }
/// ```
#[proc_macro]
pub fn model(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let model = parser::ModelParser::new().parse(&content).unwrap();

  let output = model.to_string();
  TokenStream::from_str(&output).unwrap()
}
