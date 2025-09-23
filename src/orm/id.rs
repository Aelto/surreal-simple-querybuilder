/// Wrapping type around a [surrealdb::RecordId]
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

  pub fn hash(&self) -> &str {
    &self.hash
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
