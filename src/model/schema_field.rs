use std::fmt::Display;

use crate::model::OriginHolder;
use crate::node_builder::ToNodeBuilder;

pub struct SchemaField<const N: usize> {
  pub identifier: &'static str,
  origin_holder: Option<OriginHolder<N>>,
}

impl<const N: usize> SchemaField<N> {
  pub const fn new(identifier: &'static str) -> Self {
    Self {
      identifier,
      origin_holder: None,
    }
  }

  pub const fn with_origin(identifier: &'static str, origin: Option<OriginHolder<N>>) -> Self {
    Self {
      identifier,
      origin_holder: origin,
    }
  }

  pub fn from_alias(self, alias: &'static str) -> SchemaField<{ N + 1 }> {
    let origin = match self.origin_holder {
      Some(h) => h,
      None => OriginHolder::new([""; N]),
    };

    let mut new_origin: [&'static str; N + 1] = [""; N + 1];
    new_origin[1..].clone_from_slice(&origin.segments);
    new_origin[0] = alias;

    SchemaField::<{ N + 1 }>::with_origin(self.identifier, Some(OriginHolder::new(new_origin)))
  }
}

impl<const N: usize> Display for SchemaField<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.origin_holder {
      Some(holder) => write!(f, "{holder}{}", self.identifier),
      None => write!(f, "{}", self.identifier),
    }
  }
}

impl<const N: usize> ToNodeBuilder for SchemaField<N> {
  fn equals_parameterized(&self) -> String {
    // special case for the schema field as it may include dots, we replace them
    // by underscores.
    format!("{self} = ${}", self.to_string().replace(".", "_"))
  }
}
