use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct From(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for From {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.from(self.0)
  }
}
