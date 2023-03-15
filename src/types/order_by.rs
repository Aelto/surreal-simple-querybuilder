use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct OrderBy<Order, T>(Order, T);

pub struct OrderDesc;
pub struct OrderAsc;

impl<T> OrderBy<OrderDesc, T> {
  pub fn desc(field: T) -> OrderBy<OrderDesc, T> {
    Self(OrderDesc, field)
  }
}

impl<T> OrderBy<OrderAsc, T> {
  pub fn asc(field: T) -> OrderBy<OrderAsc, T> {
    Self(OrderAsc, field)
  }
}

impl<'a> QueryBuilderInjecter<'a> for OrderBy<OrderDesc, &'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.order_by_desc(self.1)
  }
}

impl<'a> QueryBuilderInjecter<'a> for OrderBy<OrderAsc, &'a str> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.order_by_asc(self.1)
  }
}
