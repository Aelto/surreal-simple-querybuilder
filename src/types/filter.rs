use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::ToNodeBuilder;
use crate::queries::QueryBuilderConsumable;

pub struct Filter(pub Value);

impl<'a> QueryBuilderConsumable<QueryBuilder<'a>> for Filter {
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    self.0.feed(querybuilder)
  }
}

impl<'a> QueryBuilderConsumable<QueryBuilder<'a>> for Value {
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let mut query = querybuilder;

    if let Some(map) = self.as_object() {
      let mut keys = map.keys();
      if let Some(first_filter) = keys.next() {
        query = query.filter(first_filter.equals_parameterized());
      }

      for key in keys {
        query = query.and(key.equals_parameterized());
      }
    }

    query
  }
}
