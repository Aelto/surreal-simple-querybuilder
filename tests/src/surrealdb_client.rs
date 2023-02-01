use std::borrow::Cow;
use std::fmt::Display;
use std::mem::take;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use surreal_simple_querybuilder::prelude::*;
use surrealdb::engine::local::Db;
use surrealdb::Response;
use surrealdb::Surreal;

//------------------------------------------------------------------------------
// STEP 0: create models and structs

#[derive(Serialize, Deserialize, Default, Debug)]
struct IUser {
  pub id: Option<String>,
  pub name: String,
  pub email: String,
}

model!(User as user_model {
  id,
  pub name,
  pub email
});

impl IntoKey<String> for IUser {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The author has no ID"))
  }
}

use surrealdb::engine::local::Mem;
use surrealdb::opt::QueryResult;
use user_model::model as user;
use user_model::User;

#[derive(Serialize, Deserialize, Default, Debug)]
struct IBook {
  pub id: Option<String>,
  pub title: String,
  pub author: Foreign<IUser>,
  pub read: bool,
}

model!(Book as book_model {
  id,
  pub title,
  pub author<User>,
  pub read
});

use book_model::model as book;
use book_model::Book;

//------------------------------------------------------------------------------
// STEP 1: create functions that connect the querybuilder to the client

pub type DbResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type SurrealClient = Surreal<Db>;

pub async fn select<'a, R>(
  client: &SurrealClient, table: &'static str, params: impl QueryBuilderInjecter<'a> + 'a,
) -> DbResult<R>
where
  R: DeserializeOwned,
  usize: QueryResult<R>,
{
  let (query, params) = surreal_simple_querybuilder::queries::select("*", table, params)?;
  let items = client.query(query).bind(params).await?.take(0)?;

  Ok(items)
}

/// Automatically construct a CREATE query based on the `filters` that were
/// passed. The filters is a json object, obtained from the `json!({})` macro
/// offered by serde_json. Each key is added in the WHERE clause of the query
///
/// # Example
/// ```rs
/// let created = create(model, self).await?;
/// ```
pub async fn create<Table, Object>(
  client: &SurrealClient, table: Table, object: &Object,
) -> DbResult<Object>
where
  Object: Serialize + DeserializeOwned + Default,
  Table: Into<Cow<'static, str>> + Serialize + Display,
{
  let item: Option<Object> = client
    .query(
      QueryBuilder::new()
        .create(table.to_string())
        .set_model(&table)?
        .build(),
    )
    .bind(object)
    .await?
    .take(0)?;

  Ok(item.unwrap_or_default())
}

pub async fn update<'a>(
  client: &SurrealClient, table: &'a str, params: impl QueryBuilderInjecter<'a> + 'a,
) -> DbResult<Response> {
  let (query, params) = surreal_simple_querybuilder::queries::update(table, params)?;
  let response = client.query(query).bind(params).await?;

  Ok(response)
}

#[tokio::test]
async fn main() -> DbResult<()> {
  let db = Surreal::new::<Mem>(()).await?;
  db.use_ns("namespace").use_db("database").await?;

  let user0 = create(
    &db,
    user,
    &IUser {
      id: None,
      email: "john.doe@mail.com".to_owned(),
      name: "John Doe".to_owned(),
    },
  )
  .await?;

  let user1 = create(
    &db,
    user,
    &IUser {
      id: None,
      email: "jean.dupont@mail.com".to_owned(),
      name: "Jean Dupont".to_owned(),
    },
  )
  .await?;

  println!("created user 0: {user0:#?}");
  println!("created user 1: {user1:#?}");

  assert!(user0.id.is_some());

  let user0_id = user0.id.as_ref().unwrap();
  let user1_id = user1.id.as_ref().unwrap();

  db.query("create User:test set name = 'name' , email = 'email' ")
    .await?;

  create_books(&db, &user0_id, 10).await?;
  create_books(&db, &user1_id, 5).await?;

  let all_books: Vec<IBook> = select(&db, &book, ()).await?;
  let user0_books: Vec<IBook> = select(&db, &book, Where(json!({ book.author: user0_id }))).await?;
  let user1_books: Vec<IBook> = select(&db, &book, Where(json!({ book.author: user1_id }))).await?;

  assert_eq!(all_books.len(), 15);
  assert_eq!(user0_books.len(), 10);
  assert_eq!(user1_books.len(), 5);

  println!("all books: {all_books:#?}");
  println!("user0 books: {user0_books:#?}");
  println!("user1 books: {user1_books:#?}");

  // let's mark a few random books as read,
  // we're using raw indices here to keep it simple:
  let books_to_read = &[&all_books[5].id, &all_books[10].id];

  for id in books_to_read {
    if let Some(id) = id {
      read_book(&db, &id).await?;
    }
  }

  let read_books: Vec<IBook> = select(&db, &book, Where(json!({ book.read: true }))).await?;
  println!("read books: {read_books:#?}");
  assert_eq!(read_books.len(), 2);

  Ok(())
}

async fn create_books(client: &SurrealClient, author_id: &str, amount: usize) -> DbResult<()> {
  for i in 0..amount {
    create(
      &client,
      book,
      &IBook {
        id: None,
        author: Foreign::new_key(author_id.to_owned()),
        title: format!("Lorem Ipsum {i}"),
        read: false,
      },
    )
    .await?;
  }

  Ok(())
}

async fn read_book(client: &SurrealClient, book_id: &str) -> DbResult<()> {
  update(client, book_id, Set(json!({ book.read: true }))).await?;

  Ok(())
}
