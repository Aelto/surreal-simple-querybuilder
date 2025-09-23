use serde::Serialize;

#[allow(unused)]
use super::WithDbModel;
/// An extension trait for anything that can be used as binds in surrealdb
/// queries.
///
/// The [WithDbModel] methods require types that implement [WithBinding] in case
/// the queries use variables.
///
/// The usual way to bind values in surrealdb queries is by using tuples:
/// ```rs
/// DB.query("select * from $record")
///   .bind(("record", RecordId::from("table", "key")))
///   .await?;
/// ```
///
/// The WithBinding trait accepts that form and allows combining them further in
/// another tuple like so:
/// ```rs
/// let bindings = (
///   ("record", RecordId::from("table", "key")),
///   ("another", RecordId::from("table", "otherkey"))
/// );
/// ```
pub trait WithBinding {
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C>;
}

impl WithBinding for () {
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    query
  }
}

impl<VALUE> WithBinding for (&'static str, VALUE)
where
  VALUE: Serialize + 'static,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = query.bind((self.0, self.1));
    query
  }
}

#[cfg(feature = "model")]
impl<VALUE, const N: usize> WithBinding for (crate::model::SchemaField<N>, VALUE)
where
  VALUE: Serialize + 'static,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = query.bind((self.0.identifier, self.1));
    query
  }
}

impl<WB1, WB2> WithBinding for (WB1, WB2)
where
  WB1: WithBinding,
  WB2: WithBinding,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = self.0.bind(query);
    let query = self.1.bind(query);
    query
  }
}

impl<WB1, WB2, WB3> WithBinding for (WB1, WB2, WB3)
where
  WB1: WithBinding,
  WB2: WithBinding,
  WB3: WithBinding,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = self.0.bind(query);
    let query = self.1.bind(query);
    let query = self.2.bind(query);
    query
  }
}

impl<WB1, WB2, WB3, WB4> WithBinding for (WB1, WB2, WB3, WB4)
where
  WB1: WithBinding,
  WB2: WithBinding,
  WB3: WithBinding,
  WB4: WithBinding,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = self.0.bind(query);
    let query = self.1.bind(query);
    let query = self.2.bind(query);
    let query = self.3.bind(query);
    query
  }
}
