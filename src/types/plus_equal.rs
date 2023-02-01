use std::collections::HashMap;

use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::to_param_value;

pub struct PlusEqual(pub Value);

impl<'a> QueryBuilderInjecter<'a> for PlusEqual {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let mut query = querybuilder;

    if let Some(map) = self.0.as_object() {
      for key in map.keys() {
        query.add_segment(key.plus_equal_parameterized());
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
