use std::fmt::Display;

use serde::Serialize;
use serde_json::Value;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::ser_to_param_value;
use super::to_param_value;

/// Append a `left = $left` comparison. The default behaviour for the right side
/// is to always be turned into a parameter that ends up in the `params` hashmap:
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let value = 10;
/// let param = Where(Equal(("id", value)));
/// let (query, params) = select("*", "users", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users WHERE id = $id");
/// assert_eq!(params.get("id"), Some(&serde_json::json!(10)));
/// ```
///
/// **NOTE**: [Equal] offers the same result as using a raw tuple `(left, right)`:
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let value = 10;
/// let param = Where(("id", value)); // <- a tuple directly
/// let (query, params) = select("*", "users", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users WHERE id = $id");
/// assert_eq!(params.get("id"), Some(&serde_json::json!(10)));
/// ```
///
/// In order to bypass the default behaviour of the right side being turned into
/// a parameter, and until trait specialization is not available, the [Sql](super::Sql)
/// type can be used to force raw SQL:
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = Where(("id", Sql("10")));
/// let (query, params) = select("*", "users", param).unwrap();
///
/// assert_eq!(query, "SELECT * FROM users WHERE id = 10");
/// assert_eq!(params.get("id"), None);
/// ```
pub struct Equal<T>(pub T);

/// Base functions for all implementations of the `QueryBuilderInjecter` trait
impl Equal<()> {
  pub(crate) fn equal_inject<'a>(
    mut querybuilder: QueryBuilder<'a>, key: &impl ToNodeBuilder,
  ) -> QueryBuilder<'a> {
    querybuilder.add_segment(key.equals_parameterized());

    querybuilder
  }

  pub(crate) fn equal_params(
    map: &mut BindingMap, key: &impl ToNodeBuilder, value: impl Serialize,
  ) -> serde_json::Result<()> {
    map.insert(key.as_param(), ser_to_param_value(value)?);

    Ok(())
  }
}

/// This impl allows for passing a raw col that won't be parameterized using the
/// [Sql] type. See the dedicated test below for more information
impl<'a> QueryBuilderInjecter<'a> for (&str, super::Sql<&'a str>) {
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.add_segment(self.0.equals(self.1 .0));
    querybuilder
  }

  fn params(self, _: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}

#[test]
fn test_queries_equal_tuple_str() {
  use crate::prelude::*;

  let param = Where(("col", Sql("value")));
  let (query, _) = select("*", "users", param).unwrap();

  assert_eq!(query, "SELECT * FROM users WHERE col = value");
}

impl<'a, Value> QueryBuilderInjecter<'a> for &(&str, Value)
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Equal::equal_inject(querybuilder, &self.0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Equal::equal_params(map, &self.0, &self.1)
  }
}

impl<'a, Value> QueryBuilderInjecter<'a> for (&str, Value)
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Equal::equal_inject(querybuilder, &self.0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Equal::equal_params(map, &self.0, &self.1)
  }
}

#[cfg(feature = "model")]
use crate::prelude::SchemaField;

#[cfg(feature = "model")]
impl<'a, Value, const N: usize> QueryBuilderInjecter<'a> for (SchemaField<N>, Value)
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Equal::equal_inject(querybuilder, &self.0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Equal::equal_params(map, &self.0, &self.1)
  }
}

/// This impl allows for passing a raw model field that won't be parameterized
/// using the [Sql] type.
#[cfg(feature = "model")]
impl<'a, const N: usize, const N1: usize> QueryBuilderInjecter<'a>
  for (SchemaField<N>, super::Sql<SchemaField<N1>>)
where
  Value: Serialize,
{
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.add_segment(self.0.equals(&self.1 .0));
    querybuilder
  }

  fn params(self, _: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}

impl<'a, Value> QueryBuilderInjecter<'a> for &[(&str, Value)]
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    (*self).iter().fold(querybuilder, |q, pair| pair.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    for pair in self {
      pair.params(map)?;
    }

    Ok(())
  }
}

impl<'a> QueryBuilderInjecter<'a> for Value {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let mut query = querybuilder;

    if let Some(map) = self.as_object() {
      for key in map.keys() {
        query = Equal::equal_inject(query, key);
      }
    }

    query
  }

  fn params(self, params: &mut BindingMap) -> serde_json::Result<()> {
    match self {
      Value::Object(map) => {
        let iter = map
          .into_iter()
          .map(|(key, value)| (key.as_param(), to_param_value(value).unwrap()));

        params.extend(iter);
      }
      _ => {}
    };

    Ok(())
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Equal<&(Key, Value)>
where
  Key: ToNodeBuilder,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Equal::equal_inject(querybuilder, &self.0 .0)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Equal::equal_params(map, &self.0 .0, &self.0 .1)
  }
}

impl<'a, Key, Value> QueryBuilderInjecter<'a> for Equal<(Key, Value)>
where
  Key: ToNodeBuilder + Display,
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    Equal(&self.0).inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Equal(&self.0).params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for Equal<Value> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

impl<'a, Value> QueryBuilderInjecter<'a> for Equal<&[(&str, Value)]>
where
  Value: Serialize,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder)
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}
