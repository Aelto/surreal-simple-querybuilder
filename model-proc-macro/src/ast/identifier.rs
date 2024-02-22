#[derive(Debug, Clone)]
pub struct Identifier {
  pub value: String,
  pub is_raw_literal: bool,
}

impl Identifier {
  pub fn to_ident(&self) -> impl quote::ToTokens + quote::IdentFragment {
    match self.is_raw_literal {
      true => quote::format_ident!("r#{}", self.value),
      false => quote::format_ident!("{}", self.value),
    }
  }
}

impl AsRef<str> for Identifier {
  fn as_ref(&self) -> &str {
    &self.value
  }
}

impl std::fmt::Display for Identifier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl PartialEq<str> for Identifier {
  fn eq(&self, other: &str) -> bool {
    self.value == other
  }
}
