use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

pub struct Set<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Set<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.set("").commas(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}
