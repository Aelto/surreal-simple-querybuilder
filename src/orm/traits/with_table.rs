/// Types that are stored in a surrealdb table can implement this trait to get
/// a set of methods about that table.
pub trait WithTable {
  fn table() -> &'static str;
}

/// Macro for quickly implementing [WithTable] for types that have their model
/// generated using the `model!` macro:
///
/// ```rs
/// surreal_simple_querybuilder::with_table!(Account);
/// ```
/// generates the following code:
/// ```rs
/// impl WithTable for Account {
///    fn table() -> &'static str {
///        &*model
///    }
///}
/// ```
#[macro_export]
macro_rules! with_table {
  ($($struct:tt)+) => {
    impl WithTable for $($struct)+ {
      fn table() -> &'static str {
        &*model
      }
    }
  };
}
