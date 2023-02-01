use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Create<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Create<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.create(self.0)
  }
}
