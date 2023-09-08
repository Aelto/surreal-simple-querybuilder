use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum LoadedValue<V, K> {
  Loaded(V),
  Key(K),

  Unloaded,
}

impl<V, K> Default for LoadedValue<V, K> {
  fn default() -> Self {
    Self::Unloaded
  }
}

impl<V, K> LoadedValue<V, K> {
  /// Access the inner value by checking if it is loaded or not, thus returning
  /// an `Option<&T>` that is `Some` if it is loaded and `None` if it isn't.
  pub fn value(&self) -> Option<&V> {
    match self {
      Self::Loaded(v) => Some(v),
      _ => None,
    }
  }

  pub fn value_mut(&mut self) -> Option<&mut V> {
    match self {
      Self::Loaded(v) => Some(v),
      _ => None,
    }
  }

  /// Consumes `Self` to get the inner value. If the enum is in any other state
  /// than `Loaded` then a `None` is returned.
  ///
  /// Depending on how the [LoadedValue] value is obtained, for example if it is
  /// obtained by a Deref from a [ForeignKey](crate::foreign_key::ForeignKey)
  /// and if the stored types do not implement `Copy`, then calling `ForeignKey::into_inner()`
  /// might be needed:
  /// ```rs
  /// let foreign = Foreign::new(User::new("John"));
  /// let user: Option<User> = foreign.into_inner().into_value();
  /// ```
  pub fn into_value(self) -> Option<V> {
    match self {
      Self::Loaded(v) => Some(v),
      _ => None,
    }
  }

  /// Consumes `Self` to get the inner key. If the enum is in any other state
  /// than `Key` then a `None` is returned.
  ///
  /// Depending on how the [LoadedValue] value is obtained, for example if it is
  /// obtained by a Deref from a [ForeignKey](crate::foreign_key::ForeignKey)
  /// and if the stored types do not implement `Copy`, then calling `ForeignKey::into_inner()`
  /// might be needed:
  /// ```rs
  /// let foreign = Foreign::new_key(Id::from("user:john")));
  /// let id: Option<Id> = foreign.into_inner().into_key();
  /// ```
  pub fn into_key(self) -> Option<K> {
    match self {
      Self::Key(k) => Some(k),
      _ => None,
    }
  }

  /// Access the inner key by checking if the foreign key is currently
  /// holding the key, thus returning a `Some<&I>` if it is one and `None`
  /// if it isn't.
  pub fn key(&self) -> Option<&K> {
    match self {
      Self::Key(i) => Some(i),
      _ => None,
    }
  }

  pub fn key_mut(&mut self) -> Option<&mut K> {
    match self {
      Self::Key(i) => Some(i),
      _ => None,
    }
  }

  /// Attempt to construct a key from a reference to the inner value. If the
  /// foreign key:
  /// - is currently holding a key then it is cloned and returned.
  /// - is currently holding a value that implements [IntoKey](crate::foreign_key::IntoKey)
  /// then it constructs the key and returns the result
  /// - is unloaded, then `Ok(None)` is returned.
  pub fn to_key(&self) -> Result<Option<K>, super::IntoKeyError>
  where
    K: Clone,
    V: super::IntoKey<K>,
  {
    match self {
      Self::Key(i) => Ok(Some(i.clone())),
      Self::Loaded(v) => v.into_key().map(|k| Some(k)),
      Self::Unloaded => Ok(None),
    }
  }

  /// Consumes `Self` to get the inner key. If the enum is in the `Loaded`
  /// variant, then the [IntoKey](crate::foreign_key::IntoKey) implementation of the value is silently called
  /// and any error during this process will cause a `None` to be returned.
  ///
  /// Depending on how the [LoadedValue] value is obtained, for example if it is
  /// obtained by a Deref from a [ForeignKey](crate::foreign_key::ForeignKey)
  /// and if the stored types do not implement `Copy`, then calling `ForeignKey::into_inner()`
  /// might be needed:
  /// ```rs
  /// let foreign = Foreign::new(User::new("John"));
  /// let id: Option<String> = foreign.into_inner().unwrap_key();
  /// ```
  pub fn unwrap_key(self) -> Option<K>
  where
    V: super::IntoKey<K>,
  {
    match self {
      Self::Key(i) => Some(i),
      Self::Loaded(v) => v.into_key().ok(),
      _ => None,
    }
  }

  /// Return whether the current ForeignKey is unloaded. Returns `false` if `self`
  /// is either a key or a loaded value.
  pub fn is_unloaded(&self) -> bool {
    match &self {
      Self::Unloaded => true,
      _ => false,
    }
  }

  /// Returns `true` if `Self` is in the `Key` state, or `false` otherwise
  pub fn is_key(&self) -> bool {
    match &self {
      Self::Key(_) => true,
      _ => false,
    }
  }

  /// Returns `true` if `Self` is in the `Loaded` state, or `false` otherwise
  pub fn is_loaded(&self) -> bool {
    match &self {
      Self::Loaded(_) => true,
      _ => false,
    }
  }

  /// Drop any data `self` may currently hold and set it to the `Loaded` variant
  /// with the given value.
  pub fn set_value(&mut self, value: V) {
    *self = Self::Loaded(value);
  }

  /// Drop any data `self` may currently hold and set it to the `Key` variant
  /// with the given identifier.
  pub fn set_key(&mut self, identifier: K) {
    *self = Self::Key(identifier);
  }

  /// Drop the currently held value and set `self` to the `Unloaded` variant.
  pub fn unload(&mut self) {
    *self = Self::Unloaded;
  }
}

impl<V, K> Serialize for LoadedValue<V, K>
where
  K: Serialize,
  V: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self {
      Self::Loaded(v) => v.serialize(serializer),
      Self::Key(i) => i.serialize(serializer),
      Self::Unloaded => Option::<K>::None.serialize(serializer),
    }
  }
}

impl<V, K> Debug for LoadedValue<V, K>
where
  V: Debug,
  K: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Loaded(arg0) => f.debug_tuple("Loaded").field(arg0).finish(),
      Self::Key(arg0) => f.debug_tuple("Key").field(arg0).finish(),
      Self::Unloaded => write!(f, "Unloaded"),
    }
  }
}
