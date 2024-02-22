use crate::prelude::Create;
use crate::prelude::Set;

use super::bindings;
use super::query;
use super::BindingMap;
use super::QueryBuilderInjecter;

/// # Example
/// ```rs
/// let set = Set(serde_json::json!({ "name": "John", "age": 10 }));
/// let (query, params) = create("User", set).unwrap();
///
/// assert_eq!("CREATE User SET age = $age , name = $name", query);
/// assert_eq!(params.get("name"), Some(&"\"John\"".to_owned()));
/// assert_eq!(params.get("age"), Some(&"10".to_owned()));
/// ```
pub fn create<'a, T>(
  what: &'static str, component: Set<T>,
) -> serde_json::Result<(String, BindingMap)>
where
  Set<T>: QueryBuilderInjecter<'a> + 'a,
{
  let params = (Create(what), component);
  let query = query(&params)?;
  let bindings = bindings(params)?;

  Ok((query, bindings))
}

#[test]
fn test_create() {
  use crate::prelude::*;
  use serde_json::Value;

  let set = Set(serde_json::json!({ "name": "John", "age": 10 }));
  let (query, params) = create("User", set).unwrap();

  assert_eq!("CREATE User SET name = $name , age = $age", query);

  assert_eq!(params.get("name"), Some(&Value::from("John".to_owned())));
  assert_eq!(params.get("age"), Some(&Value::from(10)));
}
