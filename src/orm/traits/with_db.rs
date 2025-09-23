pub trait WithDb<C: surrealdb::Connection> {
  fn db<'a>() -> &'a surrealdb::Surreal<C>;
}

/// Macro for quickly implementing [WithDB] **as long as**:
/// - there is a `DbConnection` type alias available at the root of the crate so the
/// macro can use `crate::DbConnection` while implementing the trait
/// - there is a static `DB` variable available at the root of the crate
///
/// An example of how to declare these two requirements:
/// ```rs
/// pub type DbConnection = surrealdb::engine::local::Db;
/// pub static DB: LazyLock<Surreal<CON>> = LazyLock::new(Surreal::init);
/// ```
///
/// ## Expected output
/// ```rs
/// with_db!(Account);
/// ```
/// generates the following code:
/// ```
/// impl WithDb<crate::DbConnection> for Account {
///   fn db<'a>() -> &'a surrealdb::Surreal<crate::DbConnection> {
///     use std::ops::Deref;
///     crate::DB.deref()
///   }
/// }
/// ```
///
#[macro_export]
macro_rules! with_db {
  ($($struct:tt)+) => {
    impl WithDb<crate::DbConnection> for $($struct)+ {
      fn db<'a>() -> &'a surrealdb::Surreal<DbConnection> {
        use std::ops::Deref;
        crate::DB.deref()
      }
    }
  };
}
