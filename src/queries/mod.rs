use std::collections::HashMap;

use crate::prelude::QueryBuilder;

mod create;
mod delete;
mod impls;
mod select;
mod update;

pub use create::create;
pub use delete::delete;
pub use select::select;
pub use update::update;

pub type BindingMap = HashMap<String, serde_json::Value>;

pub trait QueryBuilderInjecter<'a> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder
  }

  fn params(self, _map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}

// TODO: this function could maybe be converted to a const fn? Or at least be
// cached
/// Constructs the query string using the supplied injecters. Refer to the individual injecters
/// for more information on how to combine them before passing them to this function or [create], or [delete], or [select], or [update].
///
/// See: [injecters](super::types) module.
pub fn query<'a>(component: &impl QueryBuilderInjecter<'a>) -> serde_json::Result<String> {
  let builder = QueryBuilder::new();
  let builder = component.inject(builder);
  let query = builder.build();

  Ok(query)
}

/// Collects the parameters out of the supplied injecters.
///
/// ```
/// use surreal_simple_querybuilder::prelude::*;
/// use serde_json::json;
///
/// let filter = Where(("id", 5));
/// let and = And(("name", "john"));
///
/// let bindings = bindings((filter, and)).unwrap();
///
/// assert_eq!(bindings.get("id"), Some(&json!(5)));
/// assert_eq!(bindings.get("name"), Some(&json!("john")));
/// ```
pub fn bindings<'a>(
  component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<BindingMap> {
  let mut params = HashMap::new();
  component.params(&mut params)?;

  Ok(params)
}
