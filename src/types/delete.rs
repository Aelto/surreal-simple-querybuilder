use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a DELETE statement to the query.
///
/// It isn't very useful by itself, especially when compared to the [delete](crate::queries::delete)
/// query function that takes care of building the query until the end.
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = Delete("user:john");
/// let query = QueryBuilder::new().injecter(&param).build();
///
/// assert_eq!(query, "DELETE user:john");
/// ```
pub struct Delete<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Delete<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.delete(self.0)
  }
}
