use std::collections::HashMap;

use crate::prelude::QueryBuilder;

mod create;
mod delete;
mod impls;
mod select;
mod update;

pub use create::create;
pub use delete::delete;
pub use impls::*;
pub use select::select;
pub use update::update;

pub trait QueryBuilderInjecter<'a> {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder
  }

  fn params(self, _map: &mut HashMap<String, String>) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}

// TODO: this function could maybe be converted to a const fn? Or at least be
// cached
pub fn query<'a>(component: &impl QueryBuilderInjecter<'a>) -> serde_json::Result<String> {
  let builder = QueryBuilder::new();
  let builder = component.inject(builder);
  let query = builder.build();

  Ok(query)
}

pub fn bindings<'a>(
  component: impl QueryBuilderInjecter<'a> + 'a,
) -> serde_json::Result<HashMap<String, String>> {
  let mut params = HashMap::new();
  component.params(&mut params)?;

  Ok(params)
}
