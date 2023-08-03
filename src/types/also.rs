use crate::queries::QueryBuilderInjecter;
use crate::querybuilder::QueryBuilder;

/// Can be used to add a comma to the query followed by a parameter or a string
///
/// # Example
/// ```rs
/// let fetch = Fetch("profile");
/// let other_fetch = Also("friends");
///
/// let (query, params) = select("*", "User", (fetch, other_fetch)).unwrap();
///
/// assert_eq!(query, "SELECT * from User FETCH profile , friends");
/// ```
pub struct Also<T>(pub T);

/// implementation for `Also` that contains a string slice,
impl<'a> QueryBuilderInjecter<'a> for Also<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.also(self.0)
  }
}

impl<'a, T> QueryBuilderInjecter<'a> for Also<T>
where
  T: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.commas(|q| self.0.inject(q))
  }

  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}
