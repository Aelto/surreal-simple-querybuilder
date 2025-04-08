use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Fetch<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Fetch<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.fetch(self.0)
  }
}

impl<'a, const N: usize> QueryBuilderInjecter<'a> for Fetch<[&'a str; N]> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.fetch_many(&self.0)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Fetch<&[&'a str]> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.fetch_many(&self.0)
  }
}
