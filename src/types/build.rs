use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// An unintuitive way to access the underlying querybuilder that is passed to
/// the querybuilder injecters. It is considered unintuitive because if you
/// access to the raw querybuilder in injecters then you may as well just
/// use the querybuilder without the injecters, but there are special cases where
/// you may want to use injecters everywhere (which is valid) and this `Build`
/// type allows you to play around the current limitations of the injecters without
/// completely giving up on them.
///
/// If the function that mutates the builder uses variable then you should also
/// use the [`Bind`] injecter to inject them.
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
