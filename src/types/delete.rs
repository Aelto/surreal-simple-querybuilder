use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Delete<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Delete<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.delete(self.0)
  }
}
