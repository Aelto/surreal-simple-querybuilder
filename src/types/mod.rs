mod create;
mod delete;
mod equal;
mod fetch;
mod filter;
mod from;
mod greater;
mod lower;
mod or;
mod pagination;
mod plus_equal;
mod select;
mod set;
mod update;

pub use create::Create;
pub use delete::Delete;
pub use equal::Equal;
pub use fetch::Fetch;
pub use filter::Where;
pub use from::From;
pub use greater::Greater;
pub use lower::Lower;
pub use or::Or;
pub use pagination::Pagination;
pub use plus_equal::PlusEqual;
pub use select::Select;
pub use set::Set;
pub use update::Update;

pub(crate) fn to_param_value(value: serde_json::Value) -> serde_json::Result<serde_json::Value> {
  Ok(value)
}

pub(crate) fn ser_to_param_value<T: serde::Serialize>(
  value: T,
) -> serde_json::Result<serde_json::Value> {
  to_param_value(serde_json::to_value(value)?)
}
