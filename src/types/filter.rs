use std::collections::HashMap;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Where<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Where<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.filter("").ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut HashMap<String, String>) -> serde_json::Result<()> {
    self.0.params(map)
  }
}
