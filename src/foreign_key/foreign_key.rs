use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde::Serialize;

use super::IntoKey;
use super::IntoKeyError;
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
#[derive(Deserialize)]
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

impl<V: Clone, K: Clone> Clone for ForeignKey<V, K> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
      allow_value_serialize: self.allow_value_serialize.clone(),
    }
  }
}

impl<V, K> ForeignKey<V, K> {
  /// Construct a new `ForeignKey` that is in the `Loaded` state holding the
  /// supplied value.
  ///
  /// [`ForeignKey<V, K>`] implements [`From<V>`] so `ForeignKey::new_value(V)`
  /// can be replaced with `ForeignKey::from(V)` or `V.into()`
  ///
  /// ```
  /// use surreal_simple_querybuilder::foreign_key::ForeignKey;
  ///
  /// let a: ForeignKey<String, String> = ForeignKey::new_value("Hello".to_owned());
  /// let b: ForeignKey<String, String> = "Hello".to_owned().into();
  ///
  /// assert_eq!(a, b);
  /// assert!(a.is_loaded());
  /// ```
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

  /// Take the owned value from this `ForeignKey`, leaving an `Unloaded` value
  /// in its place.
  ///
  /// - If the foreign key is in the `Loaded(v)` state then `Some(v)` is returned
  /// and the foreign key is put into the `Unloaded` state.
  /// - If the foreign key is in any other state then it is untouched and `None`
  /// is returned instead
  pub fn take_value(&mut self) -> Option<V> {
    if !self.inner.is_loaded() {
      return None;
    }

    std::mem::replace(&mut self.inner, LoadedValue::Unloaded).into_value()
  }

  /// Take the owned key from this `ForeignKey`, leaving an `Unloaded` value
  /// in its place.
  ///
  /// - If the foreign key is in the `Key(v)` state then `Some(v)` is returned
  /// and the foreign key is put into the `Unloaded` state.
  /// - If the foreign key is in any other state then it is untouched and `None`
  /// is returned instead
  pub fn take_key(&mut self) -> Option<K> {
    if !self.is_key() {
      return None;
    }

    std::mem::replace(&mut self.inner, LoadedValue::Unloaded).into_key()
  }

  /// Map the current value of type `V` (if it is loaded) into a new value using
  /// the provided [`mapper`] function. The resulting value of the mapper
  /// function can be of any type and not necessarily `V`.
  ///
  /// If the foreign key is not in the loaded state then the mapper function
  /// will not run.
  ///
  /// # Example
  /// ```rs
  /// let foreign_int = ForeignKey::new_value(5);
  /// let foreign_str = foreign_int.map(|n| String::from(n));
  /// ```
  pub fn map<F, NEWV>(self, mapper: F) -> ForeignKey<NEWV, K>
  where
    F: FnOnce(V) -> NEWV,
  {
    match self.inner {
      LoadedValue::Loaded(v) => ForeignKey::new_value(mapper(v)),
      LoadedValue::Key(k) => ForeignKey::new_key(k),
      LoadedValue::Unloaded => ForeignKey::new(),
    }
  }

  /// Short-hand for `foreignkey.map(NewValueType::from)`. Convert a
  /// `ForeignKey<V,K>` to a `ForeignKey<NEWV,K>` as long as `NEWV` implements
  /// `From<V>`.
  ///
  /// Refer to the [map](#map) function for more information.
  pub fn convert<NEWV>(self) -> ForeignKey<NEWV, K>
  where
    NEWV: From<V>,
  {
    self.map(NEWV::from)
  }
}

impl<V, K> ForeignKey<V, K>
where
  V: IntoKey<K>,
{
  /// Set the foreign key to hold a key if it was currently holding a value.
  pub fn as_key(&mut self) -> Result<(), IntoKeyError> {
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
      (LoadedValue::Loaded(v), false) => v
        .into_key()
        .map_err(|intokeyerr| serde::ser::Error::custom(intokeyerr))?
        .serialize(serializer),
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

impl<V, K> From<V> for ForeignKey<V, K> {
  fn from(value: V) -> Self {
    Self::new_value(value)
  }
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

  /// Appends the item to the back of the inner collection.
  /// - If `Self` is in the [LoadedValue::Unloaded] state then it is changed to
  /// [LoadedValue::Loaded] with the supplied `value` in it.
  /// - If `Self` is in the [LoadedValue::Key] state then the supplied `value`
  /// is pushed to the inner list of keys after  it is turned into a key using
  /// the [IntoKey] trait.
  /// - If `Self` is in the [LoadedValue::Loaded] state then the supplied `value`
  /// is directly pushed to the list of values with no prior transformation.
  pub fn push(&mut self, value: V) -> Result<(), IntoKeyError>
  where
    V: IntoKey<K>,
  {
    if self.is_unloaded() {
      self.inner = LoadedValue::Loaded(vec![value]);
    } else if let Some(ref mut v) = self.inner.key_mut() {
      v.push(value.into_key()?);
    } else if let Some(ref mut v) = self.inner.value_mut() {
      v.push(value);
    }

    Ok(())
  }

  /// Removes the last element from the inner collection.
  /// - If `Self` is in the [LoadedValue::Unloaded] state then nothing is done
  /// and `Ok(None)` is returned.
  /// - If `Self` is in the [LoadedValue::Key] state then the last element is
  /// popped and returned.
  /// - If `Self` is in the [LoadedValue::Loaded] state then the last element
  /// is popped, mapped to a key and returned .
  pub fn pop(&mut self) -> Result<Option<K>, IntoKeyError>
  where
    V: IntoKey<K>,
  {
    if self.is_unloaded() {
      return Ok(None);
    } else if let Some(ref mut v) = self.inner.key_mut() {
      return Ok(v.pop());
    } else if let Some(ref mut v) = self.inner.value_mut() {
      let value = v.pop();

      return match value {
        Some(value) => Ok(Some(value.into_key()?)),
        None => Ok(None),
      };
    }

    Ok(None)
  }

  /// Easily convert a `ForeignVec<V>` of values into a `ForeignVec<NEWV>`
  pub fn convert_vec<NEWV>(self) -> ForeignKey<Vec<NEWV>, Vec<K>>
  where
    NEWV: From<V>,
  {
    self.map(|values| values.into_iter().map(NEWV::from).collect())
  }
}
