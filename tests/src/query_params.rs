use serde_json::json;
use surreal_simple_querybuilder::model;
use surreal_simple_querybuilder::prelude::*;

model!(User { id, name, age });
use schema::model;

#[test]
fn test_select_fn() {
  let (q, _bindings) = select("*", &model, ()).unwrap();
  assert_eq!("SELECT * FROM User", q);

  let (q, _bindings) = select("*", &model, Where(json!({ model.name: "John" }))).unwrap();
  assert_eq!("SELECT * FROM User WHERE name = $name", q);

  let filter = Where((
    json!({ model.name: "John" }),
    Greater(json!({ model.age: 10 })),
  ));
  let (q, _bindings) = select("*", &model, filter).unwrap();
  assert_eq!("SELECT * FROM User WHERE name = $name AND age > $age", q);
}
