use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// An indirect way to access the underlying querybuilder that is passed to
/// the injecters in special cases where you may use injecters everywhere but need
/// to bypass an eventual limitation around them:
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param_where = Where(("id", Sql("5")));
/// let param_build = Build(|querybuilder: QueryBuilder| querybuilder.and("name = 'john'"));
/// let query = query(&(param_where, param_build)).unwrap();
///
/// assert_eq!(query, "WHERE id = 5 AND name = 'john'");
/// ```
pub struct Build<T>(pub T)
where
  T: Fn(QueryBuilder) -> QueryBuilder;

impl<'a, T> QueryBuilderInjecter<'a> for Build<T>
where
  T: Fn(QueryBuilder) -> QueryBuilder,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0(querybuilder)
  }
}
