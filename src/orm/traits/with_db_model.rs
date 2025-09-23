#![allow(async_fn_in_trait)]

use super::super::Id;
use super::super::ModelError;
use super::super::ModelResult;

use super::WithBinding;
use super::WithCrudEvents;
use super::WithDb;
use super::WithId;
use super::WithTable;

use crate::queries::QueryBuilderInjecter;

pub trait WithDbModel<C: surrealdb::Connection>:
  Sized
  + serde::Serialize
  + serde::de::DeserializeOwned
  + WithCrudEvents
  + WithId
  + WithTable
  + WithDb<C>
  + 'static
{
  async fn create(mut self) -> ModelResult<Self> {
    // if the random ID is not wanted, then it should be updated in the
    // [on_create_before] event.
    self.set_id(Id::from_table_key(Self::table(), Self::generate_key()));
    self.on_create_before().await?;

    let Some(item) = Self::db().create(Self::table()).content(self).await? else {
      return Err(ModelError::CreateButObtainedNone);
    };

    Ok(item)
  }

  async fn update(mut self) -> ModelResult<Self> {
    self.on_create_before().await?;
    let Some(item) = Self::db().update(self.record()).content(self).await? else {
      return Err(ModelError::UpdateButObtainedNone);
    };

    Ok(item)
  }

  async fn delete(mut self) -> ModelResult<Option<Self>> {
    self.on_delete_before().await?;
    Ok(Self::db().delete(self.record()).await?)
  }

  async fn find_by_id(id: Id) -> ModelResult<Option<Self>> {
    Ok(Self::db().select(id.record()).await?)
  }

  async fn find_by_key(key: &str) -> ModelResult<Option<Self>> {
    Ok(Self::db().select((Self::table(), key)).await?)
  }

  async fn find_many<'a>(
    query: impl QueryBuilderInjecter<'a>, binds: impl WithBinding,
  ) -> ModelResult<Vec<Self>> {
    use crate::types::*;

    let select = (Select("*"), From(Self::table()), query);
    let query = crate::queries::query(&select);

    #[cfg(feature = "orm_debug")]
    println!("find_many(), query={query:?}");

    find_many::<Self, C>(&query, binds).await
  }

  async fn find_one<'a>(
    query: impl QueryBuilderInjecter<'a>, binds: impl WithBinding,
  ) -> ModelResult<Option<Self>> {
    use crate::types::*;

    let select = (Select("*"), From(Self::table()), query);
    let query = crate::queries::query(&select);

    #[cfg(feature = "orm_debug")]
    println!("find_one(), query={query:?}");

    find_one::<Self, C>(&query, binds).await
  }
}

async fn find_many<T, C>(query: &str, binds: impl WithBinding) -> ModelResult<Vec<T>>
where
  T: serde::de::DeserializeOwned + WithDb<C>,
  C: surrealdb::Connection,
{
  let query = binds.bind(T::db().query(query));
  let items = query.await?.take(0)?;

  Ok(items)
}

async fn find_one<T, C>(query: &str, binds: impl WithBinding) -> ModelResult<Option<T>>
where
  T: serde::de::DeserializeOwned + WithDb<C>,
  C: surrealdb::Connection,
{
  let query = binds.bind(T::db().query(query));
  let item = query.await?.take(0)?;

  Ok(item)
}
