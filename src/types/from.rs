use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a `FROM` clause
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = (Select("*"), From("users"));
/// assert_eq!(query(&param).unwrap(), "SELECT * FROM users");
/// ```
pub struct From(pub &'static str);

impl<'a> QueryBuilderInjecter<'a> for From {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.from(self.0)
  }
}
