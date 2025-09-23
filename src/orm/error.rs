pub type ModelResult<T> = Result<T, ModelError>;

#[derive(Debug)]
pub enum ModelError {
  SurrealDb(surrealdb::Error),
  CreateButObtainedNone,
  UpdateButObtainedNone,
  InternalError,
  EventError(&'static str),
}

impl From<surrealdb::Error> for ModelError {
  fn from(value: surrealdb::Error) -> Self {
    Self::SurrealDb(value)
  }
}
