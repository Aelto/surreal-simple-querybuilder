use crate::types::From;
use crate::types::Select;

use super::bindings;
use super::query;
use super::BindingMap;
use super::QueryBuilderInjecter;

pub fn select<'a>(
  what: &'static str, from: &'static str, component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<(String, BindingMap)> {
  let params = (Select(what), From(from), component);
  let query = query(&params)?;
  let bindings = bindings(params)?;

  Ok((query, bindings))
}

#[test]
fn test_select() {
  use crate::prelude::*;
  use serde_json::Value;

  let filter = serde_json::json!({ "name": "John", "age": 10 });
  let pagination = Pagination::from(10..25);
  let fetch = Fetch(["friends", "articles"]);
  let components = (Where(filter), pagination, fetch);

  let (query, params) = select("*", "User", components).unwrap();

  assert_eq!(
    "SELECT * FROM User WHERE age = $age AND name = $name LIMIT 15 START AT 10 FETCH friends , articles",
    query
  );

  assert_eq!(params.get("name"), Some(&Value::from("John".to_owned())));
  assert_eq!(params.get("age"), Some(&Value::from(10)));

  let (query, params) = select(
    "*",
    "User",
    Where((
      Or(serde_json::json!({
        "one": 1,
        "two": 2
      })),
      ("three", 3),
    )),
  )
  .unwrap();

  assert_eq!(
    "SELECT * FROM User WHERE one = $one OR two = $two AND three = $three",
    query
  );
  assert_eq!(params.get("one"), Some(&Value::from(1)));
  assert_eq!(params.get("two"), Some(&Value::from(2)));
  assert_eq!(params.get("three"), Some(&Value::from(3)));
}
