use crate::prelude::Delete;

use super::bindings;
use super::query;
use super::BindingMap;
use super::QueryBuilderInjecter;

/// ```
/// use surreal_simple_querybuilder::prelude::*;
/// use serde_json::json;
///
/// let filter = Where(serde_json::json!({ "name": "John", "age": 10 }));
/// let (query, params) = delete("User", filter).unwrap();
///
/// assert_eq!("DELETE User WHERE name = $name AND age = $age", query);
/// assert_eq!(params.get("name"), Some(&json!("John")));
/// assert_eq!(params.get("age"), Some(&json!(10)));
///
/// let (query, params) = delete("User:john", ()).unwrap();
///
/// assert_eq!("DELETE User:john", query);
/// assert!(params.is_empty());
/// ```
pub fn delete<'a, 'b>(
  table: &'static str, component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<(String, BindingMap)> {
  let params = (Delete(table), component);

  Ok((query(&params)?, bindings(params)?))
}
