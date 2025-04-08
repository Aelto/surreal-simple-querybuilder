use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a `SELECT` clause
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = (Select("*"), From("users"));
/// assert_eq!(query(&param).unwrap(), "SELECT * FROM users");
/// ```
pub struct Select(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for Select {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.select(self.0)
  }
}
