use std::collections::HashMap;

use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;

pub struct Greater(pub Value);

impl<'a> QueryBuilderInjecter<'a> for Greater {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let mut query = querybuilder;

    if let Some(map) = self.0.as_object() {
      for key in map.keys() {
        query.add_segment(key.greater_parameterized());
      }
    }

    query
  }

  fn params(self, params: &mut HashMap<String, String>) -> serde_json::Result<()> {
    match self.0 {
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