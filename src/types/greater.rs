use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::ser_to_param_value;

pub struct Greater<T>(pub T);

/// Base functions for all implementations of the `QueryBuilderInjecter` trait
impl Greater<()> {
  fn greater_inject<'a>(
    mut querybuilder: QueryBuilder<'a>, key: &impl ToNodeBuilder,
  ) -> QueryBuilder<'a> {
    querybuilder.add_segment(key.greater_parameterized());

    querybuilder
  }

  fn greater_params(
    map: &mut BindingMap, key: &impl ToNodeBuilder, value: impl Serialize,
  ) -> serde_json::Result<()> {
    map.insert(key.as_param(), ser_to_param_value(value)?);

    Ok(())
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Greater<&(Key, Value)>
where
  Key: ToNodeBuilder,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Greater::greater_inject(querybuilder, &self.0 .0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Greater::greater_params(map, &self.0 .0, &self.0 .1)
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Greater<(Key, Value)>
where
  Key: ToNodeBuilder + Display,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Greater(&self.0).inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Greater(&self.0).params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Greater<Value> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    if let Some(map) = self.0.as_object() {
      return map
        .keys()
        .fold(querybuilder, |q, key| Greater::greater_inject(q, key));
    }

    querybuilder
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

impl<'a, Value> QueryBuilderInjecter<'a> for Greater<&[(&str, Value)]>
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}
