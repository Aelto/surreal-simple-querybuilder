use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Delete(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for Delete {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.delete(self.0)
  }
}
