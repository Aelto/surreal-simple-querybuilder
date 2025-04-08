use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Appends a `ORDER BY` clause that is either ASC or DESC.
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::prelude::*;
///
/// let param = OrderBy(OrderDesc, "created_at");
/// let q = query(&param).unwrap();
/// assert_eq!(q, "ORDER BY created_at DESC");
///
/// // you can also use the available methods to construct it:
/// let asc = OrderBy::asc("created_at");
/// let desc = OrderBy::desc("created_at");
/// assert_eq!(q, query(&desc).unwrap());
/// assert_eq!(query(&asc).unwrap(), "ORDER BY created_at ASC");
/// ```
pub struct OrderBy<Order, T>(pub Order, pub T);

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

#[cfg(feature = "model")]
use crate::model::SchemaField;

#[cfg(feature = "model")]
impl<'a, const N: usize> QueryBuilderInjecter<'a> for OrderBy<OrderDesc, SchemaField<N>> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.order_by_desc(self.1.to_string())
  }
}
#[cfg(feature = "model")]
impl<'a, const N: usize> QueryBuilderInjecter<'a> for OrderBy<OrderAsc, SchemaField<N>> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.order_by_asc(self.1.to_string())
  }
}
