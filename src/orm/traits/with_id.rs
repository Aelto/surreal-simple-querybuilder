pub use super::super::Id;

/// Types with an ID field can implement this trait for a set of methods around
/// getting & setting it.
pub trait WithId {
  /// returns a reference to the id field of `self`
  fn id(&self) -> &Id;
  fn set_id(&mut self, id: Id);

  /// returns the `id` part of `table:id` ids for surrealdb records. The default
  /// implementation uses nanoid to generate a UUID.
  fn generate_key() -> impl Into<surrealdb::RecordIdKey> {
    nanoid::nanoid!()
  }

  /// returns the "hash" part of the [surrealdb::RecordId], as surrealdb record
  /// IDs are a combination of `table:record-key` in this case the hash is the
  /// `record-id`
  fn hash(&self) -> &str {
    self.id().hash()
  }

  fn record(&self) -> &surrealdb::RecordId {
    &self.id().record()
  }

  fn to_record(&self) -> surrealdb::RecordId {
    self.record().clone()
  }
}

/// Macro for quickly implementing [WithId] for types that possess an `id` field
/// of type [Id].
///
/// ```rs
/// surreal_simple_querybuilder::with_id!(Account);
/// ```
/// generates the following code:
/// ```rs
/// impl WithId for Account {
///     fn id(&self) -> &Id {
///         &self.id
///     }

///     fn set_id(&mut self, id: Id) {
///         self.id = id;
///     }
/// }
/// ```
#[macro_export]
macro_rules! with_id {
    ($($struct:tt)+) => {
        impl WithId for $($struct)+ {
            fn id(&self) -> &Id {
                &self.id
            }

            fn set_id(&mut self, id: Id) {
                self.id = id;
            }
        }
    };
}

impl WithId for Id {
  fn id(&self) -> &Id {
    self
  }

  fn set_id(&mut self, id: Id) {
    *self = id;
  }
}

// conflicting implementation unfortunately...
//
// impl<MODEL> crate::foreign_key::IntoKey<Id> for MODEL
// where
//   MODEL: WithId,
// {
//   fn into_key(&self) -> Result<types::Id, surreal_simple_querybuilder::foreign_key::IntoKeyError> {
//     Ok(self.id().clone())
//   }
// }
