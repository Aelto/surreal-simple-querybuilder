#[cfg(test)]
mod test;

/// Contains a trait for simplifying the building of relationships between nodes
pub mod node_builder;

/// Contains the query builder for simplifying the building of Surreal QL queries.
/// Particularely useful when composing variables and conditional queries
pub mod querybuilder;

/// Contains the `node!` macro to quickly define static strings mapped to a
/// struct-like declaration.
pub mod node;

/// Contains the `Foreign<T>` type used to represent fields that may or may not be
/// loaded.
// pub mod foreign;
pub mod foreign_key;

/// A `ForeignKey` whose `Key` type is set to a `String` by default.
pub type Foreign<T> = foreign_key::ForeignKey<T, String>;

pub mod prelude {
  pub use super::foreign_key::*;
  pub use super::node;
  pub use super::node_builder::*;
  pub use super::querybuilder::QueryBuilder;
  pub use super::Foreign;
}
