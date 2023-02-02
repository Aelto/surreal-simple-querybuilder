use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Or<T>(pub T);

impl<'a, T> QueryBuilderInjecter<'a> for Or<T>
where
  T: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.ors(|q| self.0.inject(q))
  }

  fn params(self, map: &mut crate::queries::BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.0.params(map)
  }
}
