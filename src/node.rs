/// An easy way to declare constant strings for database schemas
///
/// # Example
/// ```
/// use surreal_simple_querybuilder::node;
///
/// node!(
///   users {
///     id,
///     handle
///   }
/// );
/// ```
/// which results in
/// ```rs
/// pub mod schema {
///   pub const label: &'static str = "users";
///   pub const id: &'static str = "id";
///   pub const handle: &'static str = "handle";
/// }
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! node {
  ($table:ident { $($column:ident),* }) => {
    pub mod schema {
      #[allow(non_upper_case_globals)]
      pub const label: &'static str = stringify!($table);

      $(
        #[allow(non_upper_case_globals)]
        pub const $column: &'static str = stringify!($column);
      )*
    }

    pub mod queries {
      /// a constant array to be used in `SELECT *` queries so that you can be
      /// sure columns are always in the same order you declared them in the
      /// macro.
      #[allow(non_upper_case_globals, unused)]
      pub const columns: &'static [&'static str] = &[$(concat!(stringify!($table), '.', stringify!($column)),)*];
    }
  };
}
