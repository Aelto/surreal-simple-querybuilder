use std::collections::HashMap;
use std::fmt::Display;

use serde::Serialize;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::prelude::ToNodeBuilder;
use crate::queries::BindingMap;

use super::Equal;

pub struct Where<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Where<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.filter("").ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

// impl<'a, Key, Value> QueryBuilderInjecter<'a> for Where<&[(Key, Value)]>
// where
//   Key: ToNodeBuilder + Display,
//   Value: Serialize,
// {
//   fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
//     for pair in self.0 {
//       querybuilder = Equal(pair).inject(querybuilder);
//       querybuilder.filter("").ands(|q| self.0.inject(q))
//     }

//     querybuilder
//   }

//   fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
//     self.0.params(map)
//   }
// }
