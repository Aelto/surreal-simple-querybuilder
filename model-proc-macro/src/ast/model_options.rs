#[derive(Debug, Default)]
pub struct ModelOptions {
  pub partial: bool,
}

impl From<Vec<super::Identifier>> for ModelOptions {
  fn from(flags: Vec<super::Identifier>) -> Self {
    Self {
      partial: flags.iter().any(|s| s == "partial"),
    }
  }
}
