use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

/// Acts as a way to send raw unaltered SQL as an injecter. It is the same as
/// doing
/// ```rs
/// QueryBuilder.raw("my string")
/// ```
pub struct Sql<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Sql<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.raw(self.0)
  }

  fn params(self, _map: &mut BindingMap) -> serde_json::Result<()> {
    Ok(())
  }
}
