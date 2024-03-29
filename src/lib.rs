#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

/// A module for the various types used & generated by the [`model!()`] proc-macro.
#[cfg(feature = "model")]
pub mod model;

#[cfg(feature = "model")]
pub use surreal_simple_querybuilder_proc_macro::model;

/// Contains a trait for simplifying the building of relationships between nodes
#[cfg(feature = "querybuilder")]
pub mod node_builder;

/// Contains the query builder for simplifying the building of Surreal QL queries.
/// Particularely useful when composing variables and conditional queries
#[cfg(feature = "querybuilder")]
pub mod querybuilder;

/// Contains the `Foreign<T>` type used to represent fields that may or may not be
/// loaded.
#[cfg(feature = "foreign")]
pub mod foreign_key;

/// Contains utility types that can be used in your rust types and that play
/// well with the querybuilder thanks to its [QueryBuilder::feed()](crate::querybuilder::QueryBuilder::feed) method and the
/// [QueryBuilderConsumable](crate::querybuilder::QueryBuilderConsumable) trait.
#[cfg(feature = "queries")]
pub mod types;

#[cfg(feature = "queries")]
pub mod queries;

pub mod prelude;

pub use serde_json;
