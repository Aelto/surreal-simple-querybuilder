use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

/// # Example
/// ```rs
/// let filter = Set(serde_json::json!({ "name": "John", "age": 10 }));
/// let (query, params) = update("User", filter).unwrap();
///
/// assert_eq!("UPDATE User SET age = $age , name = $name", query);
/// assert_eq!(params.get("name"), Some(&Value::from("John".to_owned())));
/// assert_eq!(params.get("age"), Some(&Value::from(10)));
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
