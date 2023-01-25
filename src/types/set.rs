use std::collections::HashMap;
use std::marker::PhantomData;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Set<'a, T: QueryBuilderInjecter<'a>> {
  component: T,

  _p: &'a PhantomData<()>,
}

impl<'a, T: QueryBuilderInjecter<'a>> Set<'a, T> {
  pub fn new(component: T) -> Self {
    Self {
      component,
      _p: &PhantomData {},
    }
  }
}

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Set<'a, T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.set("").commas(|q| self.component.inject(q))
  }

  fn params(self, map: &mut HashMap<String, String>) -> serde_json::Result<()> {
    self.component.params(map)
  }
}
