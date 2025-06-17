use surreal_simple_querybuilder::prelude::*;

#[test]
#[cfg(feature = "sql_standard")]
#[cfg(feature = "queries")]
#[cfg(feature = "querybuilder")]
fn test_joins() {
  let query = QueryBuilder::new()
    .select("username".distinct())
    .from("users")
    .join_left("permissions")
    .on("users.id".equals("permissions.user_id"))
    .build();

  assert_eq!(
    query,
    "SELECT DISTINCT username FROM users LEFT JOIN permissions ON users.id = permissions.user_id"
  );
}

#[test]
#[cfg(feature = "sql_standard")]
#[cfg(feature = "queries")]
#[cfg(feature = "querybuilder")]
#[cfg(feature = "model")]
fn test_model() {
  surreal_simple_querybuilder::model!(TestModel1 {
    id,
    pub username
  });
  use schema::model;

  let params = (Select(Distinct(model.username)), From(&model));
  let query = query(&params);

  assert_eq!(query, "SELECT DISTINCT username FROM TestModel1");
}
