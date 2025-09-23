use crate::model::SchemaField;
use crate::queries::QueryBuilderInjecter;
use crate::querybuilder::QueryBuilder;

pub struct Fetch<T>(pub T);

#[cfg(feature = "model")]
impl<'a, const N: usize> QueryBuilderInjecter<'a> for Fetch<SchemaField<N>> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.fetch(self.0.to_string())
  }
}

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
