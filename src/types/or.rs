use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a `OR` statement followed by anything inside the [Or].
/// If the [Or] contains multiple items then `OR`s will be added **between** them
/// but not in front.
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let filter = Where("id = 5");
/// let q = query(&(filter, Or("id = 10"))).unwrap();
/// assert_eq!(q, "WHERE id = 5 OR id = 10");
///
/// // passing multiple items to Or tells it to no longer append a OR in front
/// // but between the items:
/// let filter = Where((
///   Or(( // <- notice the Or is INSIDE the Where, as OR is added between the items now
///     ("id", Sql("5")),
///     ("id", Sql("10")),
///     ("id", Sql("15"))
///   ))
/// ));
///
/// let q = query(&filter).unwrap();
/// assert_eq!(q, "WHERE id = 5 OR id = 10 OR id = 15");
/// ```
pub struct Or<T>(pub T);

impl<'a, T> QueryBuilderInjecter<'a> for Or<T>
where
  T: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.ors(|q| self.0.inject(q))
  }

  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Or<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.or(self.0)
  }

  fn params(self, _map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}
