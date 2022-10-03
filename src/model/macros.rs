#[allow(unused_imports)]
use crate::model::SchemaField;

/// Declare a model using a struct-like syntax. The macro generates a struct
/// whose fields are named the way you declare them, but are [SchemaField].
#[allow(unused_macros)]
#[macro_export]
macro_rules! model {
  ($table:ident { $($columns:tt $(< $foreign_type:ident >)?),* }) => {
    pub mod schema {
      use super::*;
      use surreal_simple_querybuilder::prelude::*;

      pub struct $table <const N: usize> {
        origin: Option<OriginHolder<N>>,

        $(
          pub $columns: SchemaField<N>,
        )*
      }
      impl<const N: usize> $table<N> {
        const label: &'static str = stringify!($table);
        pub const fn new() -> Self {
          Self {
            origin: None,
            $(
              $columns: SchemaField::new(stringify!($columns)),
            )*
          }
        }

        pub fn with_origin(origin: OriginHolder<N>) -> Self {
          let origin = Some(origin);

          Self {
            $(
              $columns: SchemaField::with_origin(stringify!($columns), origin.clone()),
            )*
            origin,
          }
        }

        $(
          $(
            field!($columns, $foreign_type);
          )?
        )*
      }

      impl<const N: usize> std::fmt::Display for $table<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{}", Self::label)
        }
      }

      impl<const N: usize> IntoQueryBuilderSegment for $table<N> {
        fn into<'b>(self, querybuilder: &mut QueryBuilder<'b>) -> QueryBuilderSegment<'b>
        where
          Self: 'b,
        {
          querybuilder.hold(self.to_string())
        }
      }

      impl<const N: usize> ToNodeBuilder for $table<N> {}

      pub const model: $table<0> = $table::new();
    }
  };


}

#[allow(unused_macros)]
#[macro_export]
macro_rules! field {
  // for fields with <ForeignType> after the name of the field
  ($name:ident, $foreign_type:ident) => {
    pub fn $name(self) -> $foreign_type<{ N + 1 }> {
      let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
      let mut new_origin: [&'static str; N + 1] = [""; N + 1];
      new_origin[..N].clone_from_slice(&origin.segments);
      new_origin[N] = self.$name.identifier;

      $foreign_type::with_origin(OriginHolder::new(new_origin))
    }
  };

  // for fields without the <ForeignType>
  ($field:ident) => {
    fn $field() {}
  };
}
