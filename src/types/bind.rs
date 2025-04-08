use serde::Serialize;

use crate::prelude::QueryBuilderInjecter;

/// Used to explicitly bind a variable in the final hashmap of bindings without
/// altering the query string in any way.
/// ```
/// use surreal_simple_querybuilder::prelude::*;
/// use serde_json::json;
///
/// let param = Bind(("my_id", 5));
/// let (query, bindings) = select("*", "users WHERE id = $my_id", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users WHERE id = $my_id");
/// assert_eq!(bindings.get("my_id"), Some(&json!(5)));
///
/// let param = Bind(json!({
///   "my_id": 5,
///   "created_at": 123456
/// }));
/// let (query, bindings) = select("*", "users WHERE id = $my_id AND created_at > $created_at", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users WHERE id = $my_id AND created_at > $created_at");
/// assert_eq!(bindings.get("my_id"), Some(&json!(5)));
/// assert_eq!(bindings.get("created_at"), Some(&json!(123456)));
/// ```
pub struct Bind<T>(pub T);

impl<'a, Key, V> QueryBuilderInjecter<'a> for Bind<(Key, V)>
where
  Key: crate::node_builder::ToNodeBuilder,
  V: Serialize,
{
  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    super::Equal::equal_params(map, &self.0 .0, self.0 .1)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Bind<serde_json::Value> {
  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}
