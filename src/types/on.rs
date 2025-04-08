use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = On(("users.id", Sql("permissions.user_id")));
/// let (query, params) = select("*", "users INNER JOIN permissions", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users INNER JOIN permissions ON users.id = permissions.user_id");
/// ```
pub struct On<T>(pub T);

impl<'a, T> QueryBuilderInjecter<'a> for On<T>
where
  T: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder.on(""))
  }

  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for On<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.on(self.0)
  }

  fn params(self, _map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}
