use crate::prelude::CowSegment;
use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderConsumable;

/// A struct that implements the `QueryBuilderConsumable` trait and that can be
/// created from anything that implements `Into<CowSegment<'static>>`: for
/// example the SchemaField type you get out of a model created by the `model`
/// macro.
pub struct Fetch {
  segments: Vec<CowSegment<'static>>,
}

impl<T> From<T> for Fetch
where
  T: Into<CowSegment<'static>>,
{
  fn from(value: T) -> Self {
    Self::new(value)
  }
}

impl Fetch {
  pub fn new<T>(field: T) -> Self
  where
    T: Into<CowSegment<'static>>,
  {
    Self {
      segments: vec![field.into()],
    }
  }

  pub fn and<T>(mut self, field: T) -> Self
  where
    T: Into<CowSegment<'static>>,
  {
    self.segments.push(field.into());

    self
  }
}

impl<'a> QueryBuilderConsumable<QueryBuilder<'a>> for Fetch {
  fn feed(self, mut querybuilder: QueryBuilder) -> QueryBuilder {
    let mut segments = self.segments.into_iter();

    if let Some(first) = segments.next() {
      querybuilder = querybuilder.fetch(first.clone());
    }

    for field in segments {
      querybuilder = querybuilder.fetch(field);
    }

    querybuilder
  }
}
