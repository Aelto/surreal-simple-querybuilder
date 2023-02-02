use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;
use crate::queries::BindingMap;

pub struct Where<T>(pub T);

impl<'a, T: QueryBuilderInjecter<'a>> QueryBuilderInjecter<'a> for Where<T> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.filter("").ands(|q| self.0.inject(q))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
    self.0.params(map)
  }
}

// impl<'a> QueryBuilderInjecter<'a> for Where<serde_json::Value> {
//   fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
//     querybuilder.filter("").ands(|q| self.0.inject(q))
//   }

//   fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
//     self.0.params(map)
//   }
// }

// /// Allow for the shorthand of `Where(Equal((Key, Value)))`
// impl<'a, Key, Value> QueryBuilderInjecter<'a> for Where<(Key, Value)>
// where
//   Key: ToNodeBuilder + Display,
//   Value: Serialize,
// {
//   fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
//     querybuilder.filter("").ands(|q| Equal(&self.0).inject(q))
//   }

//   fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
//     Equal(&self.0).params(map)
//   }
// }

// impl<'a, Key, Value> QueryBuilderInjecter<'a> for Where<&[(Key, Value)]>
// where
//   Key: ToNodeBuilder + Display,
//   Value: Serialize,
// {
//   fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
//     querybuilder
//       .filter("")
//       .ands(|mut q| self.0.iter().fold(q, |q, pair| Equal(pair).inject(q)))
//   }

//   fn params(self, map: &mut BindingMap) -> serde_json::Result<()> {
//     Equal(&self.0).params(map)
//   }
// }
