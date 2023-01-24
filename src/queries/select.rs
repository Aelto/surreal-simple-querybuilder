use std::collections::HashMap;

use crate::prelude::*;

use super::QueryResult;

pub struct Select<T: Into<CowSegment<'static>>>(pub T);

impl<T: Into<CowSegment<'static>>> From<T> for Select<T> {
  fn from(value: T) -> Self {
    Select(value)
  }
}

// -----------------------------------------------------------------------------

pub fn select<T: Into<CowSegment<'static>>>(table: T) -> QueryResult {
  (super::query(Select(table)), HashMap::new())
}

pub fn select_params<'a, Table, Params>(table: Table, params: Params) -> QueryResult
where
  Table: Into<CowSegment<'static>>,
  Params: QueryBuilderConsumable<QueryBuilder<'a>>,
{
  let query = QueryBuilder::new()
    .feed(&Select(table))
    .feed(&params)
    .build();

  let a = params;

  (query, HashMap::new())
}

// -----------------------------------------------------------------------------

// pub trait SelectParams {
//   fn with_select<'a, T: Into<CowSegment<'static>>>(self, select: Select<T>) -> QueryBuilder<'a>;
// }

// impl SelectParams for Filter {
//   fn with_select<'a, T: Into<CowSegment<'static>>>(self, select: Select<T>) -> QueryBuilder<'a> {
//     (select, self)
//   }
// }

// impl SelectParams for Pagination {
//   fn with_select<'a, T: Into<CowSegment<'static>>>(self, select: Select<T>) -> QueryBuilder<'a> {
//     (select, self).feed()
//   }
// }

// -----------------------------------------------------------------------------

impl<'a, T: Into<CowSegment<'static>>> QueryBuilderConsumable<QueryBuilder<'a>> for Select<T> {
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let select = self;
    let segment: CowSegment = select.0.into();

    querybuilder.select(segment)
  }
}

impl<'a, T: Into<CowSegment<'static>>> QueryBuilderConsumable<QueryBuilder<'a>>
  for (Select<T>, Filter)
{
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let (select, filter) = self;

    querybuilder.feed(&select).feed(&filter)
  }
}

impl<'a, T: Into<CowSegment<'static>>> QueryBuilderConsumable<QueryBuilder<'a>>
  for (Select<T>, Pagination)
{
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let (select, pagination) = self;

    querybuilder.feed(&select).feed(&pagination)
  }
}

impl<'a, T: Into<CowSegment<'static>>> QueryBuilderConsumable<QueryBuilder<'a>>
  for (Select<T>, Filter, Pagination)
{
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let (select, filter, pagination) = self;

    querybuilder.feed(&select).feed(&filter).feed(&pagination)
  }
}

impl<'a, T: Into<CowSegment<'static>>> QueryBuilderConsumable<QueryBuilder<'a>>
  for (Select<T>, Filter, Pagination, Fetch)
{
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let (select, filter, pagination, fetch) = self;

    querybuilder
      .feed(&select)
      .feed(&filter)
      .feed(&pagination)
      .feed(&fetch)
  }
}
