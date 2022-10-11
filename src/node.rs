use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::prelude::IntoKey;

#[derive(Debug)]
pub struct Node<T, I = String> {
  pub id: Option<I>,

  inner: T,
}

impl<T, I> Deref for Node<T, I> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T, I> DerefMut for Node<T, I> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<T, I> IntoKey<I> for Node<T, I>
where
  I: Clone,
{
  fn into_key<E>(&self) -> Result<I, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(I::clone)
      .ok_or(serde::ser::Error::custom("Missing ID during serialization"))
  }
}

impl<T, I> Default for Node<T, I>
where
  T: Default,
{
  fn default() -> Self {
    Self {
      id: None,
      inner: Default::default(),
    }
  }
}

impl<T, I> Display for Node<T, I>
where
  T: Display,
  I: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.id {
      Some(id) => write!(f, "{} -> {}", id, self.inner),
      None => write!(f, "Anonymous -> {}", self.inner),
    }
  }
}

impl<T, I> Node<T, I> {
  pub fn anonymous(inner: T) -> Self {
    Self { id: None, inner }
  }

  pub fn new(id: I, inner: T) -> Self {
    Self {
      id: Some(id),
      inner,
    }
  }
}
