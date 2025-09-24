#[allow(unused)]
use super::WithId;

/// Wrapping type around a [surrealdb::RecordId], with a pre-extracted & cached
/// "hash" (the `id` part of `td:id` surreal IDs).
///
///
/// [Id] implements the [WithId] trait, which means that if your functions use [Id]
/// parameters you can potentially replace them with `id: &impl WithId` so you're
/// able to pass either an Id directly or the whole struct that implements WithId
/// rather than being limited to just the Id.
#[derive(Debug, Clone)]
pub struct Id {
  inner: surrealdb::RecordId,
  hash: String,
}

impl Id {
  pub fn from_table_key<S, K>(table: S, key: K) -> Self
  where
    S: Into<String>,
    K: Into<surrealdb::RecordIdKey>,
  {
    Self::new(surrealdb::RecordId::from_table_key(table, key))
  }

  pub fn new(record: surrealdb::RecordId) -> Self {
    Self {
      hash: record.key().to_string(),
      inner: record,
    }
  }

  /// returns the right part of this id, otherwise known as the id or hash.
  ///
  /// ```
  /// use surreal_simple_querybuilder::orm::Id;
  /// let id = Id::from_table_key("account", "abcdef");
  /// assert_eq!(id.hash(), "abcdef");
  ///
  /// let id = Id::from_table_key("account", "abc/def");
  /// assert_eq!(id.hash(), "⟨abc/def⟩");
  /// ```
  pub fn hash(&self) -> &str {
    &self.hash
  }

  /// returns a trimmed version of the key/hash of this id, excluding the
  /// delimiter characters like `⟨⟩` or backquotes.
  ///
  /// ```
  /// use surreal_simple_querybuilder::orm::Id;
  /// let id = Id::from_table_key("account", "abcdef");
  /// assert_eq!(id.hash(), "abcdef");
  /// assert_eq!(id.hash_trimmed(), "abcdef");
  ///
  /// let id = Id::from_table_key("account", "abc/def");
  /// assert_eq!(id.hash(), "⟨abc/def⟩");
  /// assert_eq!(id.hash_trimmed(), "abc/def");
  /// ```
  pub fn hash_trimmed(&self) -> &str {
    &self.hash.trim_matches(&['⟨', '⟩', '`'])
  }

  pub fn record(&self) -> &surrealdb::RecordId {
    &self.inner
  }

  pub fn unknown() -> Self {
    Self::default()
  }

  pub fn to_inner(&self) -> surrealdb::RecordId {
    self.inner.clone()
  }
}

impl Default for Id {
  fn default() -> Self {
    Self::from_table_key("unknown", "unknown")
  }
}

impl AsRef<surrealdb::RecordId> for Id {
  fn as_ref(&self) -> &surrealdb::RecordId {
    &self.inner
  }
}

impl serde::Serialize for Id {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.inner.serialize(serializer)
  }
}

impl<'de> serde::Deserialize<'de> for Id {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let record = surrealdb::RecordId::deserialize(deserializer)?;

    Ok(Self::new(record))
  }
}
