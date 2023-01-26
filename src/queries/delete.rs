use std::collections::HashMap;

use crate::prelude::Delete;

use super::bindings;
use super::query;
use super::QueryBuilderInjecter;

pub fn delete<'a, 'b>(
  table: &'static str, component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<(String, HashMap<String, String>)> {
  let params = (Delete(table), component);

  Ok((query(&params)?, bindings(params)?))
}

#[test]
fn test_delete() {
  use crate::prelude::*;

  let filter = Where(serde_json::json!({ "name": "John", "age": 10 }));
  let (query, params) = delete("User", filter).unwrap();

  assert_eq!("DELETE User WHERE age = $age AND name = $name", query);
  assert_eq!(params.get("name"), Some(&"\"John\"".to_owned()));
  assert_eq!(params.get("age"), Some(&"10".to_owned()));

  let (query, params) = delete("User:john", ()).unwrap();

  assert_eq!("DELETE User:john", query);
  assert!(params.is_empty());
}
