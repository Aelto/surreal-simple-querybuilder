mod bind;
mod build;
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

pub use bind::Bind;
pub use build::Build;
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
pub use order_by::OrderBy;
pub use pagination::Pagination;
pub use plus_equal::PlusEqual;
pub use select::Select;
pub use set::Set;
pub use sql::Sql;
pub use update::Update;

pub(crate) fn to_param_value(value: serde_json::Value) -> serde_json::Result<serde_json::Value> {
  Ok(value)
}

/// An internal function made public so the macro generate code can use it. Allows
/// the macro code to use serde_json functions without the parent crates to import
/// the serde_json crate directly.
pub fn ser_to_param_value<T: serde::Serialize>(value: T) -> serde_json::Result<serde_json::Value> {
  to_param_value(serde_json::to_value(value)?)
}
