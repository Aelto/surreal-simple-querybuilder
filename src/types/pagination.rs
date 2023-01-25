use std::ops::Range;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

pub struct Pagination(pub Range<u64>);

impl From<Range<u64>> for Pagination {
  fn from(value: Range<u64>) -> Self {
    Pagination(value)
  }
}

impl Pagination {
  pub fn new(range: Range<u64>) -> Self {
    Pagination(range)
  }

  pub fn limit(&self) -> u64 {
    self.0.end - self.0.start
  }

  pub fn start(&self) -> u64 {
    self.0.start
  }
}

impl<'a> QueryBuilderInjecter<'a> for Pagination {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    let start = self.start();

    querybuilder
      .limit(self.limit().to_string())
      .if_then(start > 0, |q| q.start_at(start.to_string()))
  }
}
