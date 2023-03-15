use std::error::Error;
use std::fmt::Display;

use serde::Serialize;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

pub struct Where<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Where<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.filter("").ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

#[derive(Debug)]
pub enum WhereFromError {
  Serialize(serde_json::Error),
  Flatten(flatten_json_object::Error),
}

impl Display for WhereFromError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WhereFromError::Serialize(e) => e.fmt(f),
      WhereFromError::Flatten(e) => e.fmt(f),
    }
  }
}

impl Error for WhereFromError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      WhereFromError::Serialize(e) => e.source(),
      WhereFromError::Flatten(e) => e.source(),
    }
  }
}

impl From<serde_json::Error> for WhereFromError {
  fn from(value: serde_json::Error) -> Self {
    Self::Serialize(value)
  }
}

impl From<flatten_json_object::Error> for WhereFromError {
  fn from(value: flatten_json_object::Error) -> Self {
    Self::Flatten(value)
  }
}

impl Where<serde_json::Value> {
  pub fn from<S: Serialize>(value: S) -> std::result::Result<Self, WhereFromError> {
    let value = serde_json::to_value(value)?;
    let flattened = flatten_json_object::Flattener::new()
      .set_key_separator(".")
      .flatten(&value)?;

    Ok(Self(flattened))
  }
}

impl<'a, Own> Where<Own>
where
  Own: QueryBuilderInjecter<'a>,
{
  pub fn extend<Other>(self, other: Other) -> Where<(Own, Other)>
  where
    Other: QueryBuilderInjecter<'a>,
  {
    Where((self.0, other))
  }

  pub fn extend_on<Other, Output>(
    self, condition: bool, other: Other,
  ) -> Where<(Own, Option<Other>)>
  where
    Other: QueryBuilderInjecter<'a>,
    Output: QueryBuilderInjecter<'a>,
  {
    match condition {
      true => Where((self.0, Some(other))),
      false => Where((self.0, None)),
    }
  }
}
