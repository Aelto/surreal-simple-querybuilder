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

impl<WB1, WB2> WithBinding for (WB1, WB2)
where
  WB1: Serialize + 'static,
  WB2: Serialize + 'static,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = query.bind(self.0);
    let query = query.bind(self.1);
    query
  }
}

impl<WB1, WB2, WB3> WithBinding for (WB1, WB2, WB3)
where
  WB1: Serialize + 'static,
  WB2: Serialize + 'static,
  WB3: Serialize + 'static,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = query.bind(self.0);
    let query = query.bind(self.1);
    let query = query.bind(self.2);
    query
  }
}

impl<WB1, WB2, WB3, WB4> WithBinding for (WB1, WB2, WB3, WB4)
where
  WB1: Serialize + 'static,
  WB2: Serialize + 'static,
  WB3: Serialize + 'static,
  WB4: Serialize + 'static,
{
  fn bind<C: surrealdb::Connection>(
    self, query: surrealdb::method::Query<C>,
  ) -> surrealdb::method::Query<C> {
    let query = query.bind(self.0);
    let query = query.bind(self.1);
    let query = query.bind(self.2);
    let query = query.bind(self.3);
    query
  }
}
