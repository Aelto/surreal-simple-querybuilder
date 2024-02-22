#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "querybuilder")]
#[cfg(feature = "queries")]
#[cfg(feature = "model")]
#[cfg(feature = "foreign")]

mod one {
  use serde::Serialize;

  surreal_simple_querybuilder::model!(TestModel0 as model_base with(partial) {
    id,
  });
}

mod two {
  use super::one::model_base::TestModel0;

  surreal_simple_querybuilder::model!(TestModel1 {
    id,
    pub r#in,
    pub other<TestModel0>,
    pub ->relation->TestModel0 as r#for
  });

  #[test]
  fn test_string_literal() {
    assert_eq!(schema::model.r#in.to_string(), "in");
    assert_eq!(schema::model.r#for.to_string(), "->relation->TestModel0");

    assert_eq!(
      serde_json::to_string(&schema::model.r#in).unwrap(),
      "\"in\""
    );

    let filter = surreal_simple_querybuilder::types::Where((schema::model.r#in, "some_value"));
    let (query, params) =
      surreal_simple_querybuilder::queries::select("*", "table", filter).unwrap();

    assert_eq!(query, "SELECT * FROM table WHERE in = $in");

    assert_eq!(
      params.get("in"),
      Some(&serde_json::to_value("some_value").unwrap())
    );
  }
}
