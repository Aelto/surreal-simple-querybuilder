use std::collections::HashMap;

use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::to_param_value;

pub struct Lower(pub Value);

impl<'a> QueryBuilderInjecter<'a> for Lower {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let mut query = querybuilder;

    if let Some(map) = self.0.as_object() {
      for key in map.keys() {
        query.add_segment(key.lower_parameterized());
      }
    }

    query
  }

  fn params(self, params: &mut BindingMap) -> serde_json::Result<()> {
    match self.0 {
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
