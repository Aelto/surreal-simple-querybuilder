use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

/// ```
/// use surreal_simple_querybuilder::prelude::*;
/// use serde_json::json;
/// let filter = Set(json!({ "name": "John", "age": 10 }));
/// let (query, params) = update("User", filter).unwrap();
///
/// assert_eq!("UPDATE User SET name = $name , age = $age", query);
/// assert_eq!(params.get("name"), Some(&json!("John")) );
/// assert_eq!(params.get("age"), Some(&json!(10)) );
/// ```
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = Set((
///   PlusEqual(("read_count", 1)),
///   ("last_read", Sql("now()"))
/// ));
/// let (query, params) = update("articles", param).unwrap();
///
/// assert_eq!(query, "UPDATE articles SET read_count += $read_count , last_read = now()");
/// assert_eq!(params.get("read_count"), Some(&serde_json::json!(1)));
/// ```
pub struct Set<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Set<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.set("").commas(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}
