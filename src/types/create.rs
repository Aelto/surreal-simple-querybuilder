use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = Create("user:john");
/// let q = query(&param).unwrap();
/// assert_eq!(q, "CREATE user:john");
/// ```
pub struct Create<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Create<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.create(self.0)
  }
}
