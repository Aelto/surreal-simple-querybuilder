use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

pub struct And<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for And<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.0.inject(querybuilder.and(""))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

impl<'a> QueryBuilderInjecter<'a> for And<&'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.and(self.0)
  }

  fn params(self, _map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}
