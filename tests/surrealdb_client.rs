#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "querybuilder")]
#[cfg(feature = "queries")]
#[cfg(feature = "model")]
#[cfg(feature = "foreign")]
mod test {

  use std::borrow::Cow;
  use std::fmt::Display;

  use serde::de::DeserializeOwned;
  use serde::Deserialize;
  use serde::Serialize;
  use serde_json::json;
  use surreal_simple_querybuilder::prelude::*;
  use surrealdb::engine::local::Db;
  use surrealdb::sql::Thing;
  use surrealdb::Response;
  use surrealdb::Surreal;

  //------------------------------------------------------------------------------
  // STEP 0: create models and structs

  #[derive(Serialize, Deserialize, Default, Debug)]
  struct IUser {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
  }

  model!(User as user_model {
    id,
    pub name,
    pub email
  });

  impl IntoKey<Thing> for IUser {
    fn into_key(&self) -> Result<Thing, IntoKeyError> {
      self
        .id
        .as_ref()
        .map(Thing::clone)
        .ok_or(IntoKeyError::Custom("The author has no ID"))
    }
  }

  use surrealdb::engine::local::Mem;
  use surrealdb::opt::QueryResult;
  use user_model::model as user;
  use user_model::User;

  #[derive(Serialize, Deserialize, Default, Debug)]
  struct IBook {
    pub id: Option<Thing>,
    pub title: String,
    pub author: ForeignKey<IUser, Thing>,
    pub read: bool,
  }

  model!(Book as book_model with(partial) {
    id,
    pub title,
    pub author<User>,
    pub read
  });

  use book_model::model as book;

  //------------------------------------------------------------------------------
  // STEP 1: create functions that connect the querybuilder to the DB client

  pub type DbResult<T> = Result<T, Box<dyn std::error::Error>>;
  pub static DB: once_cell::sync::Lazy<Surreal<surrealdb::engine::local::Db>> =
    once_cell::sync::Lazy::new(Surreal::init);

  pub async fn connect_db() -> DbResult<()> {
    DB.connect::<Mem>(()).await?;
    DB.use_ns("namespace").use_db("database").await?;

    Ok(())
  }

  pub async fn select<'a, R>(
    table: &'static str, params: impl QueryBuilderInjecter<'a> + 'a,
  ) -> DbResult<R>
  where
    R: DeserializeOwned,
    usize: QueryResult<R>,
  {
    let (query, params) = surreal_simple_querybuilder::queries::select("*", table, params)?;
    let items = bind_params(DB.query(query), params).await?.take(0)?;

    Ok(items)
  }

  pub async fn create<Table, Object>(table: Table, object: &Object) -> DbResult<Object>
  where
    Object: Serialize + DeserializeOwned + Default,
    Table: Into<Cow<'static, str>> + Serialize + Display,
  {
    // Note how it doesn't use the params but instead use the model to know which
    // field it should include in the object:
    let item: Option<Object> = DB
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
    table: &'a str, params: impl QueryBuilderInjecter<'a> + 'a,
  ) -> DbResult<Response> {
    let (query, params) = surreal_simple_querybuilder::queries::update(table, params)?;
    let response = bind_params(DB.query(query), params).await?;

    Ok(response)
  }

  /// There is currently a rough edge between the bindings from the querybuilder
  /// and surrealdb itself because of the Serialize impl of [surrealdb::sql::Thing]
  fn bind_params<N: surrealdb::Connection>(
    mut query: surrealdb::method::Query<N>,
    params: std::collections::HashMap<String, serde_json::Value>,
  ) -> surrealdb::method::Query<N> {
    for (key, value) in params {
      match value {
        serde_json::Value::Object(mut obj) => {
          if obj.contains_key("id") && obj.contains_key("tb") {
            use serde_json::Value;
            use surrealdb::sql::Id;

            let Some(Value::String(tb)) = obj.remove("tb") else {
              continue;
            };

            let Some(Value::Object(mut id)) = obj.remove("id") else {
              continue;
            };

            let Some(Value::String(id)) = id.remove("String") else {
              continue;
            };

            query = query.bind((
              key,
              surrealdb::sql::Thing {
                id: Id::from(id),
                tb: tb,
              },
            ));
          } else {
            query = query.bind((key, obj));
          }
        }
        _ => {
          query = query.bind((key, value));
        }
      };
    }

    query
  }

  //------------------------------------------------------------------------------
  // STEP 2: use the functions

  #[tokio::test]
  async fn main() -> DbResult<()> {
    connect_db().await?;

    let user0 = create(
      user,
      &IUser {
        id: None,
        email: "john.doe@mail.com".to_owned(),
        name: "John Doe".to_owned(),
      },
    )
    .await?;

    let user1 = create(
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

    create_books(&user0_id, 10).await?;
    create_books(&user1_id, 5).await?;

    let all_books: Vec<IBook> = select(&book, ()).await?;
    let user0_books: Vec<IBook> = select(&book, Where((book.author, user0_id))).await?;
    let user1_books: Vec<IBook> = select(&book, Where((book.author, user1_id))).await?;
    let both_users_books: Vec<IBook> = select(
      &book,
      Where(Or(json!({
        book.author: user0_id,
        book.author: user1_id
      }))),
    )
    .await?;

    assert_eq!(all_books.len(), 15);
    assert_eq!(user0_books.len(), 10);
    assert_eq!(user1_books.len(), 5);

    println!("all books: {all_books:#?}");
    println!("user0 books: {user0_books:#?}");
    println!("user1 books: {user1_books:#?}");
    println!("both users books: {both_users_books:#?}");

    // let's mark a few random books as read,
    // we're using raw indices here to keep it simple:
    let books_to_read = &[&all_books[5].id, &all_books[10].id];

    for id in books_to_read {
      if let Some(id) = id {
        read_book(&id).await?;
      }
    }

    let read_books: Vec<IBook> = select(&book, Where((book.read, true))).await?;
    println!("read books: {read_books:#?}");
    assert_eq!(read_books.len(), 2);

    let books_with_author: Vec<IBook> = select(
      &book,
      (
        Where((book.author, user1_id)),
        Fetch([book.author.as_ref()]),
      ),
    )
    .await?;

    assert!(!books_with_author.is_empty());

    if let Some(first) = books_with_author.first() {
      assert!(!first.author.is_unloaded());

      if let Some(author) = first.author.value() {
        assert_eq!(author.id, user1.id);
      }
    }

    println!("books with author: {books_with_author:#?}");

    Ok(())
  }

  async fn create_books(author_id: &Thing, amount: usize) -> DbResult<()> {
    for i in 0..amount {
      create(
        book,
        &IBook {
          id: None,
          author: ForeignKey::new_key(author_id.to_owned()),
          title: format!("Lorem Ipsum {i}"),
          read: false,
        },
      )
      .await?;
    }

    Ok(())
  }

  async fn read_book(book_id: &Thing) -> DbResult<()> {
    let filter = Where((book.id, book_id));
    let set = Set((book.read, true));
    update(&book_id.tb, (set, filter)).await?;

    Ok(())
  }
}
