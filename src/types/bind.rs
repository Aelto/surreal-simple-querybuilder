use serde::Serialize;

use crate::prelude::QueryBuilderInjecter;

/// Used to explicitly bind a variable
/// ```rs
/// Bind(("key", "value"));
/// Bind(serde_json::json!({
///   "key": "value",
///   "foo": 10
/// }));
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
