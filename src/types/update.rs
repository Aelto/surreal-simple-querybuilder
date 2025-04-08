use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a `UPDATE` clause
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = (Update("users"), Where(("id", 5)));
/// assert_eq!(query(&param).unwrap(), "UPDATE users WHERE id = $id");
/// ```
pub struct Update<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Update<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.update(self.0)
  }
}
