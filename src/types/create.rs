use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Create(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for Create {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.create(self.0)
  }
}
