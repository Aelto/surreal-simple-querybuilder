use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::Serialize;

use super::IntoKey;
use super::KeySerializeControl;
use super::LoadedValue;

/// Represents foreign data, from a foreign node that may need to be fetched
/// during the query or else it won't be loaded or it will simply be the ID to a
/// foreign node.
///
/// A [ForeignKey] field may be in one of the following forms:
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
/// # ForeignKeys and serialize
/// By default a ForeignKey does not serialize its value if it is in the Loaded
/// state. The value would be transformed into a key using the [IntoKey]
/// trait methods before serializing it.
///
/// There are cases where this behaviour is not what you wish to happen, calling
/// [`ForeignKey::allow_value_serialize()`] flags the ForeignKey to serialize any
/// potential value it may hold.
///
/// **Note** that if you plan to use `ForeignKey<T, String>` (where the second generic
/// type is a string), you can use the `Foreign<T>` type in the same module to
/// shorten the declaration.
#[derive(Deserialize, Clone)]
#[serde(from = "LoadedValue<V, K>")]
pub struct ForeignKey<V, K> {
  inner: LoadedValue<V, K>,

  #[serde(skip)]
  allow_value_serialize: OnceCell<bool>,
}

impl<V, K> Default for ForeignKey<V, K> {
  fn default() -> Self {
    Self {
      inner: Default::default(),
      allow_value_serialize: OnceCell::new(),
    }
  }
}

impl<V, K> Deref for ForeignKey<V, K> {
  type Target = LoadedValue<V, K>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<V, K> DerefMut for ForeignKey<V, K> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<V, K> ForeignKey<V, K> {
  pub fn new_value(value: V) -> Self {
    Self {
      inner: LoadedValue::Loaded(value),
      ..Default::default()
    }
  }

  pub fn new_key(key: K) -> Self {
    Self {
      inner: LoadedValue::Key(key),
      ..Default::default()
    }
  }

  pub fn new() -> Self {
    Self {
      inner: LoadedValue::Unloaded,
      ..Default::default()
    }
  }

  pub fn into_inner(self) -> LoadedValue<V, K> {
    self.inner
  }
}

impl<V, K> ForeignKey<V, K>
where
  V: IntoKey<K>,
{
  pub fn to_key<E>(&mut self) -> Result<(), E>
  where
    E: serde::ser::Error,
  {
    if let Some(value) = self.value() {
      self.inner.set_key(value.into_key()?);
    }

    Ok(())
  }
}

impl<V, K> KeySerializeControl for ForeignKey<V, K> {
  fn allow_value_serialize(&self) {
    if let Err(_) = self.allow_value_serialize.set(true) {}
  }

  fn disallow_value_serialize(&mut self) {
    self.allow_value_serialize = OnceCell::new();
  }
}

impl<V, K> Serialize for ForeignKey<V, K>
where
  V: IntoKey<K>,
  K: Serialize,
  V: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match (
      &self.inner,
      self.allow_value_serialize.get().unwrap_or(&false),
    ) {
      (LoadedValue::Loaded(v), false) => v.into_key()?.serialize(serializer),
      (inner, _) => inner.serialize(serializer),
    }
  }
}

impl<V, K> From<LoadedValue<V, K>> for ForeignKey<V, K> {
  fn from(value: LoadedValue<V, K>) -> Self {
    Self {
      inner: value,
      ..Default::default()
    }
  }
}

impl<V, K> Debug for ForeignKey<V, K>
where
  V: Debug,
  K: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.inner.fmt(f)
  }
}

/// Custom implementation of PartialEq as the allow_value_serialize flag should
/// NOT be used during the comparison
impl<V, K> PartialEq for ForeignKey<V, K>
where
  V: PartialEq,
  K: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl<V, K> Eq for ForeignKey<V, K>
where
  V: Eq,
  K: Eq,
{
}

impl<V, K> ForeignKey<Vec<V>, Vec<K>> {
  /// Custom implementation of a `len` function to get the length of the inner
  /// vectors. If the ForeignKey is in the `Unloaded` state then 0 is returned.
  ///
  /// If you wish to know when no length is available then use the `len_loaded()`
  /// function
  pub fn len(&self) -> usize {
    self.len_loaded().unwrap_or_default()
  }

  /// Returns the length of the inner vectors if they are loaded, which means that
  /// self must be either a vector of keys or a vector of values. If self is in
  /// the `Unloaded` state then `None` is returned.
  ///
  /// If you wish to get a raw `usize` that defaults to `0` when the ForeignVec is
  /// unloaded then use the `len()` function
  pub fn len_loaded(&self) -> Option<usize> {
    match (self.key(), self.value()) {
      (Some(v), _) => Some(v.len()),
      (_, Some(v)) => Some(v.len()),
      _ => None,
    }
  }
}
