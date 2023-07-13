use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

/// Add a WHERE clause to the query, the `Where` type is made to accept anything
/// that implements the [QueryBuilderInjecter] trait, meaning any of the injecter
/// types that come with the crate + your own.
///
/// # Examples
/// ```rs
/// // a single field:
/// let filter = Where(("username", "John"));
///
/// // if you use the `model` macro:
/// let filter = Where((schema.username, "John"));
/// ```
///
/// ```rs
/// // multiple fields:
/// let filter = Where(json!({ "username": "John", schema.slug: "john-doe" }));
///
/// // or using the shorter alias macro:
/// let filter = wjson!({ "username": "John", schema.slug: "john-doe" });
/// ```
///
/// # Note
/// Both the json macro (that results in a `serde_json::Value`) and
/// the tuple can work when inside a `Where` because they both work the same way
/// as the [Equal] injecter. In the same style, passing an `Option<T>` as the value
/// can be used to pass an optional filter, where the whole key/value pair will
/// be ignored on a `None`
pub struct Where<T>(pub T);

/// An alias macro for
/// ```rs
/// Where(json!({ foo: bar }));
/// ```
///
/// # Example
/// ```rs
/// let filter = wjson!({ foo: bar });
/// ```
#[macro_export]
macro_rules! wjson {
    ($($json:tt)+) => {
        surreal_simple_querybuilder::types::Where(surreal_simple_querybuilder::serde_json::json!($($json)+))
    };
}

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Where<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.filter("").ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

impl<'a, Own> Where<Own>
where
  Own: QueryBuilderInjecter<'a>,
{
  pub fn extend<Other>(self, other: Other) -> Where<(Own, Other)>
  where
    Other: QueryBuilderInjecter<'a>,
  {
    Where((self.0, other))
  }

  pub fn extend_on<Other, Output>(
    self, condition: bool, other: Other,
  ) -> Where<(Own, Option<Other>)>
  where
    Other: QueryBuilderInjecter<'a>,
    Output: QueryBuilderInjecter<'a>,
  {
    match condition {
      true => Where((self.0, Some(other))),
      false => Where((self.0, None)),
    }
  }
}
