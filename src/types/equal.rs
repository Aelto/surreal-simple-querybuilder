use std::collections::HashMap;

use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;

pub struct Equal(pub Value);

impl<'a> QueryBuilderInjecter<'a> for Equal {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder)
  }

  fn params(self, map: &mut HashMap<String, String>) -> serde_json::Result<()> {
    self.0.params(map)
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

  fn params(self, params: &mut HashMap<String, String>) -> serde_json::Result<()> {
    match self {
      Value::Object(map) => {
        let iter = map
          .into_iter()
          .map(|(key, value)| (key, serde_json::to_string(&value).unwrap()));

        params.extend(iter);
      }
      _ => {}
    };

    Ok(())
  }
}
