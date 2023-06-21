use std::ops::Range;

use crate::prelude::QueryBuilder;
use crate::prelude::QueryBuilderInjecter;

/// Declare a LIMIT and START AT clause that will include the items from the
/// supplied range. The [`Self::new_page(page: u64, page_size: u64)`] function
/// offers an easy way to construct a range with a given page & page size.
///
/// _The START AT clause is omitted if the left side of the range is lower or
/// equal than 0._
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

  /// Create a new [Pagination] for the given `page` where each page contains
  /// `page_size` elements. This function assumes the pagination starts at page
  /// 0, so `Pagination::new_page(1, 20)` is the second page with elements from
  /// the `20..40` range.
  pub fn new_page(page: u64, page_size: u64) -> Self {
    Self::new(page * page_size..(page + 1) * page_size)
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
