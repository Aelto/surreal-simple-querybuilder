use std::error::Error;
use std::fmt::Display;

use serde::Serialize;

use crate::queries::QueryBuilderInjecter;

pub trait IntoOptionalInjecterExt
where
  Self: Sized,
{
  /// Helper function to turn the current injecter into a `None` if the passed
  /// condition is false, or into `Some(Self)` if the condition is true.
  ///
  /// # Example
  /// ```rs
  /// let fetch_author = true;
  /// let fetch = Fetch(["author"]).when(fetch_author);
  ///
  /// // is the equivalent of:
  /// let fetch = fetch_author.then(|| Some(Fetch(["author"])));
  /// ```
  fn when(self, condition: bool) -> Option<Self>;
}

impl<'a, Injecters> IntoOptionalInjecterExt for Injecters
where
  Injecters: QueryBuilderInjecter<'a>,
{
  fn when(self, condition: bool) -> Option<Self> {
    match condition {
      true => Some(self),
      false => None,
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
/// Function  to easily serialize+flatten serializable types into injecters that
/// accept the `Value` type

#[derive(Debug)]
pub enum FlattenSerializeError {
  Serialize(serde_json::Error),
  Flatten(flatten_json_object::Error),
}

impl Display for FlattenSerializeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FlattenSerializeError::Serialize(e) => e.fmt(f),
      FlattenSerializeError::Flatten(e) => e.fmt(f),
    }
  }
}

impl Error for FlattenSerializeError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      FlattenSerializeError::Serialize(e) => e.source(),
      FlattenSerializeError::Flatten(e) => e.source(),
    }
  }
}

impl From<serde_json::Error> for FlattenSerializeError {
  fn from(value: serde_json::Error) -> Self {
    Self::Serialize(value)
  }
}

impl From<flatten_json_object::Error> for FlattenSerializeError {
  fn from(value: flatten_json_object::Error) -> Self {
    Self::Flatten(value)
  }
}

pub fn flatten_serialize(
  value: impl Serialize,
) -> std::result::Result<serde_json::Value, FlattenSerializeError> {
  let value = serde_json::to_value(value)?;
  let flattened = flatten_json_object::Flattener::new()
    .set_key_separator(".")
    .flatten(&value)?;

  Ok(flattened)
}
