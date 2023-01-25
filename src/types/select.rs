use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Select(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for Select {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.select(self.0)
  }
}
