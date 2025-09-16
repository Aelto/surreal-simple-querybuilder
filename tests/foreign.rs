#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[test]
#[cfg(feature = "foreign")]
fn foreign_key_impl_eq() {
  use surreal_simple_querybuilder::prelude::*;

  #[derive(Clone, PartialEq, Debug)]
  struct TestA(pub &'static str);

  #[derive(Clone, Debug, PartialEq)]
  struct TestB {
    field: Foreign<TestA>,
  }

  // 0.
  // confirm comparison works as expected for a value vs a key
  assert_ne!(
    TestB {
      field: Foreign::new_value(TestA("lorem")),
    },
    TestB {
      field: Foreign::new_key("key".to_owned()),
    }
  );

  // 1.
  // confirm comparison works as expected for two different values
  assert_ne!(
    TestB {
      field: Foreign::new_value(TestA("lorem")),
    },
    TestB {
      field: Foreign::new_value(TestA("ipsum")),
    }
  );

  // 2.
  // confirm comparison works as expected for two identical values
  assert_eq!(
    TestB {
      field: Foreign::new_value(TestA("lorem")),
    },
    TestB {
      field: Foreign::new_value(TestA("lorem")),
    }
  );

  // 3.
  // confirm comparison works as expected for two identical keys
  assert_eq!(
    TestB {
      field: Foreign::new_key("key".to_owned())
    },
    TestB {
      field: Foreign::new_key("key".to_owned())
    }
  );

  // 4.
  // confirm two unloaded values are considered equal
  assert_eq!(
    TestB {
      field: Foreign::new()
    },
    TestB {
      field: Foreign::new()
    }
  );
}

/// This test is more of a "failsafe" just to ensure the Clone implementation is
/// not removed from the ForeignKey type by mistake.
///
/// ... It also relies heavily on the implementation of Eq
#[test]
#[cfg(feature = "foreign")]
fn foreign_key_impl_clone() {
  use surreal_simple_querybuilder::prelude::*;

  #[derive(Clone, PartialEq, Debug)]
  struct TestA(pub &'static str);

  #[derive(Clone, Debug, PartialEq)]
  struct TestB {
    field: Foreign<TestA>,
  }

  let original = TestB {
    field: Foreign::new_value(TestA("lorem")),
  };

  let cloned = original.clone();

  assert_eq!(original, cloned);
}

#[test]
#[cfg(feature = "foreign")]
fn foreign_flatten_prefix_loaded_value() {
  use serde::Deserialize;
  use serde::Serialize;
  use serde_with::with_prefix;
  use surreal_simple_querybuilder::prelude::*;

  with_prefix!(prefix_permissions_join "permissions_");

  #[derive(Serialize, Deserialize)]
  struct User {
    id: String,

    #[serde(flatten, with = "prefix_permissions_join")]
    #[serde(skip_serializing_if = "Foreign::is_not_value")]
    permissions: Foreign<UserPermissions>,
  }

  #[derive(Serialize, Deserialize)]
  struct UserPermissions {
    id: String,
    is_admin: bool,
    is_guest: bool,
  }

  impl IntoKey<String> for UserPermissions {
    fn into_key(&self) -> Result<String, surreal_simple_querybuilder::prelude::IntoKeyError> {
      Ok(self.id.clone())
    }
  }

  let user = User {
    id: "users:1".to_owned(),
    permissions: Foreign::new_value(UserPermissions {
      id: "permissions:1".to_owned(),
      is_admin: false,
      is_guest: true,
    }),
  };

  user.permissions.allow_value_serialize();

  let json = serde_json::to_string(&user).unwrap();
  assert_eq!(json, "{\"id\":\"users:1\",\"permissions_id\":\"permissions:1\",\"permissions_is_admin\":false,\"permissions_is_guest\":true}");

  let parsed_user: User = serde_json::from_str(&json).unwrap();

  assert_eq!(parsed_user.id, user.id);
  assert_eq!(parsed_user.permissions.is_loaded(), true);
  if let Some(permissions) = parsed_user.permissions.value() {
    let original = user.permissions.value().unwrap();

    assert_eq!(permissions.id, original.id);
    assert_eq!(permissions.is_admin, original.is_admin);
    assert_eq!(permissions.is_guest, original.is_guest);
    assert_eq!(permissions.is_guest, true);
  }

  let user = User {
    id: "users:2".to_owned(),
    permissions: Foreign::new_key("permissions:2".to_owned()),
  };

  let json = serde_json::to_string(&user).unwrap();
  dbg!(json);
}
