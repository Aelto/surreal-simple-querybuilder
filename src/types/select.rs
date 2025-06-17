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
pub struct Select<T>(pub T);

impl<'a> QueryBuilderInjecter<'a> for Select<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.select(self.0)
  }
}

impl<'a, COL> QueryBuilderInjecter<'a> for Select<super::Distinct<COL>>
where
  super::Distinct<COL>: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder.select(""))
  }
}
