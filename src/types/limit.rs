use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Dynamically add a limit statement to the query.
/// ```rs
/// Limit(10);
/// Limit("10");
/// ```
///
/// **Note:** If you know the limit value at compile time prefer a
/// `&'static str` over a `u64` to avoid an unnecessary `to_string()` call.
///
pub struct Limit<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Limit<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.limit(self.0)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Limit<u64> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.limit(self.0.to_string())
  }
}
