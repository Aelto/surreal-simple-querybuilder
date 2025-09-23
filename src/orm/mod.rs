//! A suite of traits and macros to give simple structs ORM-like methods for
//! most CRUD operations.
//!
//! ## Setting it all up
//! ### Setting up the database client
//! ```rs
//! use std::sync::LazyLock;
//! use surrealdb::Surreal;
//!
//! // the name of these two items are used by the macros, they must not be changed
//! pub type DbConnection = surrealdb::engine::local::Db;
//! pub static DB: LazyLock<Surreal<DbConnection>> = LazyLock::new(Surreal::init);
//!
//! pub async fn connect(
//!     address: &str,
//!     _username: &str,
//!     _password: &str,
//!     namespace: &str,
//!     database: &str,
//! ) {
//!     DB.connect::<surrealdb::engine::local::SurrealKV>(address)
//!       .await
//!       .expect("database init: failed to connect");
//!
//!     DB.use_ns(namespace)
//!         .use_db(database)
//!         .await
//!         .expect("database init: failed to use ns & db");
//! }
//! ```
//! ### Implementing the ORM for our types
//! ```rs
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct IAccount {
//!     id: Id,
//!     creation_date: chrono::DateTime<chrono::Utc>,
//!
//!     pub handle: Handle,
//!     pub password: Password,
//! }
//!
//! surreal_simple_querybuilder::with_orm!(IAccount);
//! surreal_simple_querybuilder::model!(Account {
//!   id,
//!   pub creation_date,
//!   pub handle,
//!   pub password
//! });
//! pub use schema::model;
//! ```
//! ### Using the ORM functions
//! ```rs
//! use crate::prelude::*;
//!
//! impl WithCrudEvents for IAccount {
//!     async fn on_create_before(&mut self) -> ModelResult<()> {
//!         self.validated_ref()
//!          .map_err(|_| ModelError::EventError("validation"))?;
//!
//!         self.creation_date = chrono::Utc::now();
//!         self.hashed_password(nanoid::nanoid!().into())
//!             .await
//!             .map_err(|_| ModelError::EventError("password_hashing_error"))?;
//!
//!         // we want the ID ouf our accounts to be generated from their username,
//!         // so for example `John Doe` becomes `account:john-doe`:
//!         let slug = self.handle.to_slug();
//!         self.set_id(Id::from_table_key(Self::table(m), slug.as_str()));
//!
//!         Ok(())
//!     }
//! }
//!
//! impl IAccount {
//!     pub fn new(handle: Handle) -> Self {
//!         Self {
//!             id: Id::default(),
//!             creation_date: chrono::Utc::now(),
//!             password: Password::default(),
//!             handle,
//!         }
//!     }
//!
//!     pub async fn find_by_handle(handle: &Handle) -> ModelResult<Option<Self>> {
//!         let filter = Where((model.handle, handle));
//!         let binds = ("handle", handle.clone());
//!
//!         Ok(Self::find_one(filter, binds).await?)
//!     }
//! }
//!
//!
//! ```
pub mod traits;

mod id;
pub use id::Id;

mod error;
pub use error::ModelError;
pub use error::ModelResult;

#[allow(unused)]
use traits::*;
/// Generate the macro generated implementation of [WithId], [WithTable], and
/// [WithDb] in order to implement [WithDbModel] for the given struct.
///
/// ```rs
/// with_orm!(Account);
/// ```
/// generates the following code:
/// ```rs
/// surreal_simple_querybuilder::with_id!($($struct)+);
/// surreal_simple_querybuilder::with_table!($($struct)+);
/// surreal_simple_querybuilder::with_db!($($struct)+);
///
/// impl WithDbModel<DbConnection> for $($struct)+ {}
/// ```
/// _refer to the individual macros for a complete preview of the generated code.
#[macro_export]
macro_rules! with_orm {
  ($($struct:tt)+) => {
    surreal_simple_querybuilder::with_id!($($struct)+);
    surreal_simple_querybuilder::with_table!($($struct)+);
    surreal_simple_querybuilder::with_db!($($struct)+);

    impl WithDbModel<DbConnection> for $($struct)+ {}
  };
}
