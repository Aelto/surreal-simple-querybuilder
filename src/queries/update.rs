use std::collections::HashMap;

use crate::prelude::Update;

use super::bindings;
use super::query;
use super::QueryBuilderInjecter;

pub fn update<'a, 'b>(
  table: &'static str, component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<(String, HashMap<String, String>)> {
  let params = (Update(table), component);

  Ok((query(&params)?, bindings(params)?))
}

#[test]
fn test() {
  use crate::prelude::Pagination;

  let filter = Set::new(serde_json::json!({ "name": "John", "age": 10 }));
  let pagination = Pagination::from(10..25);
  let components = (filter, pagination);

  let (query, params) = update("User", components).unwrap();

  dbg!(query, params);
}
