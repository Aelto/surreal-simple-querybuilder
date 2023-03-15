#[derive(Debug, Default)]
pub struct ModelOptions {
  pub partial: bool,
}

impl From<Vec<String>> for ModelOptions {
  fn from(flags: Vec<String>) -> Self {
    Self {
      partial: flags.iter().any(|s| s == "partial"),
    }
  }
}
