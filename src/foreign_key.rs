use std::fmt::Debug;
use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;

/// Represents foreign data, from a foreign node that may need to be fetched
/// during the query or else it won't be loaded or it will simply be the ID to a
/// foreign node.
///
/// A [ForeignKey] field may have one of the following values:
///  - Loaded data,
///  - An ID,
///  - None of the above (`null`)
///
/// When a field is set as a `ForeignKey<V, K>` or a `Foreign<V>`, the field will
/// always be serialized into an ID so you can be sure you won't get raw data
/// inserted into your nodes by mistake.
///
/// Pairs well with objects that store IDs in the surreal DB, that you can also
/// load using the `FETCH` keyword of SurrealQL.
///
/// Imagining the following structure:
/// ```sql
/// create User:John set name = "John";
/// create File set name = "John file", author = User:John;
/// ```
///
/// which could be represented like so in Rust:
/// ```rs
/// struct User {
///   name: String
/// }
///
/// struct File {
///   name: String,
///   author: ForeignKey<User, String>
/// }
/// ```
///
/// This will cause the serde_json library to attempt to parse the `File::author`
/// as a `User`, and if it fails will then attempt to parse it as a `String` type
/// (a string in our case since this is how SurrealDB stores IDs). And if the
/// attempt to parse the ID fails as well it will default to the `Unloaded` variant
/// of a ForeignKey
///
/// You are then free to use the ForeignKey's methods to safely access the foreign
/// data
/// ```rs
/// let file: File; // = query("SELECT * from File FETCH author");
///
/// if let Some(user) = file.author.value() {
///   // the file had an author and it was loaded
///   dbg!(&user);
/// }
///
/// if let Some(user_id) = file.author.key() {
///   // the file had an author ID, but it wasn't loaded
///   dbg!(&user_id);
/// }
/// ```
///
/// **Note** that if you plan to use `ForeignKey<T, String>` (where the second generic
/// type is a string), you can use the `Foreign<T>` type in the same module to
/// shorten the declaration.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ForeignKey<V, K> {
  Loaded(V),
  Key(K),

  Unloaded,
}

impl<V, K> Default for ForeignKey<V, K> {
  fn default() -> Self {
    Self::Unloaded
  }
}

impl<V, K> ForeignKey<V, K>
where
  V: Debug,
  V: IntoKey<K>,
{
  /// Access the inner value by checking if it is loaded or not, thus returning
  /// an `Option<&T>` that is `Some` if it is loaded and `None` if it isn't.
  pub fn value(&self) -> Option<&V> {
    match self {
      Self::Loaded(v) => Some(v),
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

  /// Return whether the current ForeignKey is unloaded. Returns `false` if `self`
  /// is either a key or a loaded value.
  pub fn is_unloaded(&self) -> bool {
    match &self {
      ForeignKey::Unloaded => true,
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

impl<V, K> Serialize for ForeignKey<V, K>
where
  V: IntoKey<K>,
  K: Serialize,
  V: Debug,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match &self {
      ForeignKey::Loaded(v) => v.into_key::<S::Error>()?.serialize(serializer),
      ForeignKey::Key(i) => i.serialize(serializer),
      ForeignKey::Unloaded => Option::<K>::None.serialize(serializer),
    }
  }
}

impl<V, K> Debug for ForeignKey<V, K>
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

/// Any type use inside a [ForeignKey] must implement this trait. It allows you
/// to transform the `I` type into an ID when `I` is serialized.
pub trait IntoKey<I> {
  fn into_key<E>(&self) -> Result<I, E>
  where
    E: serde::ser::Error;
}

/// An implementation for vector of objects that implement the IntoKey
/// trait. So you can have `Foreign<Vec<MyType>>` rather than `Vec<Foreign<MyType>>`
impl<V, K> IntoKey<K> for Vec<V>
where
  V: IntoKey<K>,
  K: std::iter::FromIterator<K>,
{
  fn into_key<E>(&self) -> Result<K, E>
  where
    E: serde::ser::Error,
  {
    self.iter().map(|c| c.into_key()).collect()
  }
}

impl<V: IntoKey<K>, K> IntoKey<K> for Box<V> {
  fn into_key<E>(&self) -> Result<K, E>
  where
    E: serde::ser::Error,
  {
    self.deref().into_key()
  }
}

/// A `ForeignKey` whose `Key` type is set to a `String` by default.
pub type Foreign<T> = ForeignKey<T, String>;
