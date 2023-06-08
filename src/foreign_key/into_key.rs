use std::{fmt::Display, ops::Deref};

#[derive(Debug, Clone, Copy)]
pub enum IntoKeyError {
  /// Denotes a IntoKey failure as `Self` had no ID to provide. Can happen on
  /// types whose IDs are `Option<Id>` and when it is currently a `None`
  MissingId,

  /// Denotes a IntoKey failure that happened while `Self` was serializing into
  /// the ID's type.
  TransformError,

  /// A custom error message
  Custom(&'static str),
}

impl Display for IntoKeyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Custom(message) => write!(f, "IntoKeyError: {message}"),
      Self::MissingId => write!(f, "IntoKeyError: MissingId"),
      Self::TransformError => write!(f, "IntoKeyError: TransformError"),
    }
  }
}

impl std::error::Error for IntoKeyError {}

/// Any type used inside a [ForeignKey] must implement this trait. It allows you
/// to transform the `I` type into an ID when `I` is serialized.
pub trait IntoKey<I> {
  fn into_key(&self) -> Result<I, IntoKeyError>;
}

impl<V, K> IntoKey<Vec<K>> for Vec<V>
where
  V: IntoKey<K>,
  Vec<K>: std::iter::FromIterator<K>,
{
  fn into_key(&self) -> Result<Vec<K>, IntoKeyError> {
    self.iter().map(|c| c.into_key()).collect()
  }
}

impl<V: IntoKey<K>, K> IntoKey<K> for Box<V> {
  fn into_key(&self) -> Result<K, IntoKeyError> {
    self.deref().into_key()
  }
}

/// A blanket implementation for `Option<V>` as long as V implements `IntoKey<K>`
/// so it is easier to implement on types that have a `id: Option<Id>` field.
impl<V: IntoKey<K>, K> IntoKey<K> for Option<V> {
  fn into_key(&self) -> Result<K, IntoKeyError> {
    match self {
      Some(id) => id.into_key(),
      None => Err(IntoKeyError::MissingId),
    }
  }
}

impl IntoKey<String> for &str {
  fn into_key(&self) -> Result<String, IntoKeyError> {
    Ok(self.to_string())
  }
}

impl IntoKey<String> for String {
  fn into_key(&self) -> Result<String, IntoKeyError> {
    Ok(self.to_owned())
  }
}
