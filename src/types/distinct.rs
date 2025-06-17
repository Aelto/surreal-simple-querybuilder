use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;

/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = (
///   Select(Distinct("username")),
///   From("users")
/// );
/// let query = query(&param).unwrap();
///
/// assert_eq!(query, "SELECT DISTINCT username FROM users");
/// ```
pub struct Distinct<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Distinct<&str> {
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.add_segment(self.0.distinct());
    querybuilder
  }
}

#[cfg(all(feature = "model", feature = "sql_standard"))]
use crate::model::SchemaField;

#[cfg(all(feature = "model", feature = "sql_standard"))]
impl<'a, const N: usize> QueryBuilderInjecter<'a> for Distinct<SchemaField<N>> {
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.add_segment(self.0.to_string().distinct());
    querybuilder
  }
}
