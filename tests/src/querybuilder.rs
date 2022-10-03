use serde::Deserialize;
use serde::Serialize;

use surreal_simple_querybuilder::prelude::*;

/// For the tests, and as an example we are creating what could be an Account in
/// a simple database.
#[derive(Debug, Serialize, Deserialize, Default)]
struct Account {
  id: Option<String>,
  handle: String,
  password: String,
  email: String,

  friends: Foreign<Vec<Account>>,
}

mod account {
  use super::*;

  model!(Account {
    handle,
    password,
    email,
    friends<Account>
  });
}

use account::schema::model as account;

#[derive(Debug, Serialize, Deserialize)]
struct File {
  name: String,
  author: Foreign<Account>,
}

impl IntoKey<String> for Account {
  fn into_key<E>(&self) -> Result<String, E>
  where
    E: serde::ser::Error,
  {
    self
      .id
      .as_ref()
      .map(String::clone)
      .ok_or(serde::ser::Error::custom("The account has no ID"))
  }
}

///
impl QueryBuilderSetObject for Account {
  fn set_querybuilder_object<'a>(mut querybuilder: QueryBuilder<'a>) -> QueryBuilder {
    let a = &[
      querybuilder.hold(account.handle.equals_parameterized()),
      querybuilder.hold(account.password.equals_parameterized()),
      querybuilder.hold(account.email.equals_parameterized()),
    ];

    querybuilder.set_many(a)
  }
}

#[test]
fn test_create_account_query() {
  let query = QueryBuilder::new()
    .create(account.handle.as_named_label(&account.to_string()))
    .set_object::<Account>()
    .build();

  assert_eq!(
    query,
    "CREATE Account:handle SET handle = $handle , password = $password , email = $email"
  );
}

#[test]
fn test_account_find_query() {
  let query = QueryBuilder::new()
    .select("*")
    .from("test")
    .filter(account.email.equals_parameterized())
    .build();

  assert_eq!(query, "SELECT * FROM Account WHERE email = $email");
}

#[test]
pub fn test_nodebuilder_relation() {
  let s = "Account".with("IS_FRIEND").with("Account:Mark").to_owned();

  assert_eq!("Account->IS_FRIEND->Account:Mark", s);
}

#[test]
pub fn test_nodebuilder_condition() {
  let should_be_friend_with_mark = true;
  let should_be_friend_with_john = false;

  let s = String::new()
    .with("IS_FRIEND")
    .if_then(should_be_friend_with_mark, |s| s.with("Account:Mark"))
    .if_then(should_be_friend_with_john, |s| s.with("Account:John"))
    .to_owned();

  assert_eq!("->IS_FRIEND->Account:Mark", s);
}

#[test]
pub fn test_as_named_label() {
  let user_handle = "John";
  let label = user_handle.as_named_label("Account");

  assert_eq!(label, "Account:John");
}

#[test]
pub fn test_foreign_serialize() {
  let f: Foreign<Account> = Foreign::Key("Account:John".to_owned());

  // Confirm a foreign key is serialized into a simple string
  assert_eq!(
    serde_json::Value::String("Account:John".to_owned()),
    serde_json::to_value(f).unwrap()
  );

  let f: Foreign<Account> = Foreign::Loaded(Account {
    id: Some("Account:John".to_owned()),
    ..Default::default()
  });

  // Confirm a loaded value uses the IntoKey trait during serialization
  assert_eq!(
    serde_json::Value::String("Account:John".to_owned()),
    serde_json::to_value(f).unwrap()
  );
}

#[test]
fn test_foreign_deserialize() {
  let created_account = Account {
    id: Some("Account:John".to_owned()),
    handle: "JohnTheUser".to_owned(),
    password: "abc".to_owned(),
    email: "abc".to_owned(),
    friends: Default::default(),
  };

  // build a json string where the author field contains a fully built Account
  // object.
  let loaded_author_json = format!(
    "{{ \"name\": \"filename\", \"author\": {} }}",
    serde_json::to_string(&created_account).unwrap()
  );

  let file: File = serde_json::from_str(&loaded_author_json).unwrap();

  // confirm the `Foreign<Author>` contains a value
  assert!(match &file.author.value() {
    Some(acc) => acc.id == Some("Account:John".to_owned()),
    _ => false,
  });

  // build a json string where the author field is an ID string.
  let key_author_json = "{ \"name\": \"filename\", \"author\": \"Account:John\" }";
  let file: File = serde_json::from_str(&key_author_json).unwrap();

  // confirm the author field of the file is a Key with the account's ID
  assert!(match file.author.key().as_deref() {
    Some(key) => key == &"Account:John".to_owned(),
    _ => false,
  });

  // build a json string where the author field is set to null.
  let unloaded_author_json = "{ \"name\": \"filename\", \"author\": null }";
  let file: File = serde_json::from_str(&unloaded_author_json).unwrap();

  // confirm the author field of the file is Unloaded
  assert!(match file.author {
    Foreign::Unloaded => true,
    _ => false,
  });
}

/// Test that a model can have fields that reference the `Self` type.
#[test]
fn test_model_self_reference() {
  assert_eq!("friends", account.friends.to_string());
  assert_eq!("Account", account.friends().to_string());
  assert_eq!("friends.handle", account.friends().handle.to_string());
}
