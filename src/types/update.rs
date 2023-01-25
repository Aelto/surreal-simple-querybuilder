use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Update(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for Update {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.update(self.0)
  }
}
