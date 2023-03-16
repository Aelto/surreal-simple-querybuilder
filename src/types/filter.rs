use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

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
