use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

/// Appends a `AND` statement followed by anything inside the [and].
/// If the [And] contains multiple items then `AND`s will be added **between** them
/// but not in front.
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let filter = Where("id = 5");
/// let q = query(&(filter, And("name = 'john'"))).unwrap();
/// assert_eq!(q, "WHERE id = 5 AND name = 'john'");
///
/// // passing multiple items to And tells it to no longer append a AND in front
/// // but between the items:
/// let filter = Where((
///   And(( // <- notice the Or is INSIDE the Where, as AND is added between the items now
///     ("id", Sql("5")),
///     ("name", Sql("'john'")),
///     ("role", Sql("'premium'"))
///   ))
/// ));
///
/// let q = query(&filter).unwrap();
/// assert_eq!(q, "WHERE id = 5 AND name = 'john' AND role = 'premium'");
/// ```
pub struct And<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for And<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for And<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.and(self.0)
  }

  fn params(self, _map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}
