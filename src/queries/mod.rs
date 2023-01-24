use std::collections::HashMap;
use std::marker::PhantomData;

use crate::prelude::*;

mod select;

pub use select::*;

pub struct Client<T>(PhantomData<T>);
pub type QueryResult = (String, HashMap<String, String>);

// impl<T: Into<CowSegment<'static>>> QueryBuilderConsumable for (Update<T>, Filter) {
//   fn compose<'a>(self) -> QueryBuilder<'a> {
//     let (update, filter) = self;

//     QueryBuilder::new().select(update.0).feed(filter)
//   }
// }

// impl<T: Into<CowSegment<'static>>> QueryBuilderConsumable for (Delete<T>, Filter) {
//   fn compose<'a>(self) -> QueryBuilder<'a> {
//     let (delete, filter) = self;

//     QueryBuilder::new().select(delete.0).feed(filter)
//   }
// }

pub struct Update<T: Into<CowSegment<'static>>>(pub T);
pub struct Delete<T: Into<CowSegment<'static>>>(pub T);

pub fn query<'a, C: QueryBuilderConsumable<QueryBuilder<'a>>>(consumable: C) -> String {
  QueryBuilder::new().feed(consumable).build()
}

pub trait QueryBuilderConsumable<T>
where
  Self: Sized,
{
  fn feed(self, consumer: T) -> T;
}

// pub trait QueryBuilderConsumer<T: QueryBuilderConsumable> {
//   fn eat(self, consumable: T) -> Self;
// }

impl<'a> QueryBuilderConsumable<QueryBuilder<'a>> for (Filter, Pagination) {
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    let (filter, pagination) = self;

    querybuilder.feed(filter).feed(pagination)
  }
}

impl<'a> QueryBuilderConsumable<QueryBuilder<'a>> for (Pagination, Filter) {
  fn feed(self, querybuilder: QueryBuilder) -> QueryBuilder {
    // re-use the `(Filter,Pagination)` impl
    (self.1, self.0).feed(querybuilder)
  }
}
