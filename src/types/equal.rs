use std::collections::HashMap;
use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::ser_to_param_value;
use super::to_param_value;

pub struct Equal<T>(pub T);

impl<'a, Key, Value> QueryBuilderInjecter<'a> for &[(Key, Value)]
where
  Key: ToNodeBuilder + Display,
  Value: Serialize,
{
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    for (key, _) in *self {
      querybuilder.add_segment(key.equals_parameterized());
    }

    querybuilder
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    for (key, value) in self {
      map.insert(key.as_param(), ser_to_param_value(&value)?);
    }

    Ok(())
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Equal<(Key, Value)>
where
  Key: ToNodeBuilder + Display,
  Value: Serialize,
{
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.add_segment(self.0 .0.equals_parameterized());

    querybuilder
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    map.insert(self.0 .0.as_param(), ser_to_param_value(self.0 .1)?);

    Ok(())
  }
}

impl<'a> QueryBuilderInjecter<'a> for Value {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let mut query = querybuilder;

    if let Some(map) = self.as_object() {
      for key in map.keys() {
        query.add_segment(key.equals_parameterized());
      }
    }

    query
  }

  fn params(self, params: &mut BindingMap) -> serde_json::Result<()> {
    match self {
      Value::Object(map) => {
        let iter = map
          .into_iter()
          .map(|(key, value)| (key.as_param(), to_param_value(value).unwrap()));

        params.extend(iter);
      }
      _ => {}
    };

    Ok(())
  }
}

impl<'a> QueryBuilderInjecter<'a> for Equal<Value> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}
