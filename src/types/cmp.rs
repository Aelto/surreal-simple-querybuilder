use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::ser_to_param_value;

type Operator = &'static str;

pub struct Cmp<T>(pub Operator, pub T);

/// Base functions for all implementations of the `QueryBuilderInjecter` trait
impl Cmp<()> {
  fn cmp_inject<'a>(
    mut querybuilder: QueryBuilder<'a>, operator: Operator, key: &impl ToNodeBuilder,
  ) -> QueryBuilder<'a> {
    querybuilder.add_segment(key.compares_parameterized(operator));

    querybuilder
  }

  fn cmp_params(
    map: &mut BindingMap, key: &impl ToNodeBuilder, value: impl Serialize,
  ) -> serde_json::Result<()> {
    map.insert(key.as_param(), ser_to_param_value(value)?);

    Ok(())
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Cmp<&(Key, Value)>
where
  Key: ToNodeBuilder,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Cmp::cmp_inject(querybuilder, self.0, &self.1 .0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Cmp::cmp_params(map, &self.1 .0, &self.1 .1)
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Cmp<(Key, Value)>
where
  Key: ToNodeBuilder + Display,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Cmp(self.0, &self.1).inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Cmp(self.0, &self.1).params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Cmp<Value> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    if let Some(map) = self.1.as_object() {
      return map
        .keys()
        .fold(querybuilder, |q, key| Cmp::cmp_inject(q, self.0, key));
    }

    querybuilder
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.1.params(map)
  }
}

impl<'a, Value> QueryBuilderInjecter<'a> for Cmp<&[(&str, Value)]>
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.1.inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.1.params(map)
  }
}
