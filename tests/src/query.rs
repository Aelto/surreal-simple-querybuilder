use serde_json::json;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::*;
use surreal_simple_querybuilder::queries::query;
use surreal_simple_querybuilder::queries::Select;

model!(User { id, name });
use schema::model;
use surreal_simple_querybuilder::queries::select;

#[test]
fn test_query_fn() {
  let q = dbg!(query(Select(model)));
  assert_eq!("SELECT User", q);

  let q = dbg!(query((
    Select(model),
    Filter(json!({ model.name: "John" }))
  )));
  assert_eq!("SELECT User WHERE name = $name", q);
}

#[test]
fn test_select() {
  let (q, _bindings) = select(model);
  assert_eq!("SELECT User", q);

  // let (q, _bindings) = select_params(model, Filter(json!({ model.name: "John" })));
  // assert_eq!("SELECT User WHERE name = $name", q);
}
