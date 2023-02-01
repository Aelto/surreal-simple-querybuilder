use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Update<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Update<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.update(self.0)
  }
}
