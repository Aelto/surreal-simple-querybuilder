use surreal_simple_querybuilder::prelude::*;

#[test]
fn foreign_key_impl_eq() {
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
fn foreign_key_impl_clone() {
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
