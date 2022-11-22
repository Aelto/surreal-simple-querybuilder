use darling::{FromMeta};

#[derive(Debug, FromMeta)]
pub struct QueryBuilderObjectDeriveOptions {
  pub model: String
}