#![allow(async_fn_in_trait)]

use super::super::ModelResult;

pub trait WithCrudEvents {
  async fn on_create_before(&mut self) -> ModelResult<()> {
    Ok(())
  }

  async fn on_update_before(&mut self) -> ModelResult<()> {
    Ok(())
  }

  async fn on_delete_before(&mut self) -> ModelResult<()> {
    Ok(())
  }
}
