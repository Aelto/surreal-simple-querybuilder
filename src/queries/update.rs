use crate::prelude::Update;

use super::bindings;
use super::query;
use super::BindingMap;
use super::QueryBuilderInjecter;

pub fn update<'a, 'b>(
  table: &'a str, component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<(String, BindingMap)> {
  let params = (Update(table), component);

  Ok((query(&params)?, bindings(params)?))
}

#[test]
fn test_update() {
  use crate::prelude::*;
  use serde_json::Value;

  let filter = Set(serde_json::json!({ "name": "John", "age": 10 }));
  let (query, params) = update("User", filter).unwrap();

  assert_eq!("UPDATE User SET age = $age , name = $name", query);
  assert_eq!(params.get("name"), Some(&Value::from("John".to_owned())));
  assert_eq!(params.get("age"), Some(&Value::from(10)));
}
