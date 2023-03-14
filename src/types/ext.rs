use crate::queries::QueryBuilderInjecter;

pub trait IntoOptionalInjecterExt
where
  Self: Sized,
{
  /// Helper function to turn the current injecter into a `None` if the passed
  /// condition is false, or into `Some(Self)` if the condition is true.
  ///
  /// # Example
  /// ```rs
  /// let fetch_author = true;
  /// let fetch = Fetch(["author"]).when(fetch_author);
  ///
  /// // is the equivalent of:
  /// let fetch = fetch_author.then(|| Some(Fetch(["author"])));
  /// ```
  fn when(self, condition: bool) -> Option<Self>;
}

impl<'a, Injecters> IntoOptionalInjecterExt for Injecters
where
  Injecters: QueryBuilderInjecter<'a>,
{
  fn when(self, condition: bool) -> Option<Self> {
    match condition {
      true => Some(self),
      false => None,
    }
  }
}
