use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// A struct that implements the `QueryBuilderInjecter` trait and that can be
/// created from anything that implements `Into<CowSegment<'static>>`: for
/// example the SchemaField type you get out of a model created by the `model`
/// macro.
// pub struct Fetch<const N: usize>(pub [&'static str; N]);

pub struct Fetch<T>(pub T);

impl<'a, const N: usize> QueryBuilderInjecter<'a> for Fetch<[&'a str; N]> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.fetch_many(&self.0)
  }
}

// impl<'a, const N: usize> QueryBuilderInjecter<'a> for Fetch<[Sch; N]> {
//   fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
//     querybuilder.fetch_many(&self.0)
//   }
// }
