use std::fmt::Debug;
use std::fmt::Display;

use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;

use super::Field;
use super::Identifier;
use super::ModelOptions;

#[derive(Debug)]
pub struct Model {
  pub name: Identifier,
  pub fields: Vec<Field>,
  pub alias: Option<Identifier>,
  pub options: ModelOptions,
}

impl Display for Model {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = self.name.to_ident();

    let field_declarations: Vec<TokenStream> =
      self.fields.iter().map(|field| field.emit_field()).collect();

    let struct_declaration = quote! {
      #[derive(serde::Serialize)]
      pub struct #name <const N: usize> {
        #[serde(skip_serializing)]
        origin: Option<OriginHolder<N>>,
        #(#field_declarations),*
      }
    };

    let partial_declaration = match self.options.partial {
      false => quote! {},
      true => {
        let partial_name = format_ident!("Partial{}", self.name.as_ref());
        let partial_declaration = quote! {
          #[derive(serde::Serialize, Debug)]
          #[serde(transparent)]
          pub struct #partial_name (
            surreal_simple_querybuilder::serde_json::Map<String, serde_json::Value>,
            #[serde(skip)] surreal_simple_querybuilder::serde_json::Result<()>,
          );
        };

        let field_setter_functions: Vec<TokenStream> = self
          .fields
          .iter()
          .map(|field| field.emit_partial_setter_field_function())
          .collect();

        quote!(
          #partial_declaration

          impl #partial_name {
            pub fn new() -> Self {
              Self(surreal_simple_querybuilder::serde_json::Map::new(), Ok(()))
            }

            fn __insert_value_result(mut self, key: &str, value: impl Serialize) -> Self {
              match surreal_simple_querybuilder::types::ser_to_param_value(value) {
                Ok(v) => {
                  self.0.insert(key.to_owned(), v);
                }
                Err(e) => {
                  self.1 = self.1.and(Err(e));
                }
              };

              self
            }


            pub fn ok(self) -> std::result::Result<serde_json::Value, surreal_simple_querybuilder::types::FlattenSerializeError> {
              self.1?;

              surreal_simple_querybuilder::types::flatten_serialize(self.0)
            }

            #(#field_setter_functions)*
          }
        )
      }
    };

    let field_assignments: Vec<TokenStream> = self
      .fields
      .iter()
      .map(|field| field.emit_initialization())
      .collect();

    let field_assignments_with_origin: Vec<TokenStream> = self
      .fields
      .iter()
      .map(|field| field.emit_initialization_with_origin())
      .collect();

    let field_foreign_functions: Vec<TokenStream> = self
      .fields
      .iter()
      .map(|field| field.emit_foreign_field_function())
      .collect();

    let implementations = quote! {
      impl<const N: usize> #name<N> {
        const label: &'static str = stringify!(#name);
        pub const fn new() -> Self {
          Self {
            origin: None,
            #(#field_assignments),*
          }
        }

        pub fn with_origin(origin: OriginHolder<N>) -> Self {
          let origin = Some(origin);

          Self {
            #(#field_assignments_with_origin),*
            ,origin,
          }
        }

        #(#field_foreign_functions)*
      }

      impl<const N: usize> std::fmt::Display for #name<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{}", Self::label)
        }
      }

      impl<const N: usize> Into<std::borrow::Cow<'static, str>> for #name<N> {
        fn into(self) -> std::borrow::Cow<'static, str> {
          std::borrow::Cow::from(Self::label)
        }
      }

      impl<const N: usize> std::ops::Deref for #name<N> {
        type Target = str;

        fn deref(&self) -> &Self::Target {
          Self::label
        }
      }

      impl<const N: usize> AsRef<str> for #name<N> {
        fn as_ref(&self) -> &str {
          Self::label
        }
      }

      impl<const N: usize> ToNodeBuilder for #name<N> {}
    };

    let module_name = match &self.alias {
      Some(alias) => format_ident!("{alias}"),
      None => format_ident!("schema"),
    };

    let output = quote! {
      pub mod #module_name {
        use super::*;
        use surreal_simple_querybuilder::prelude::*;

        #struct_declaration
        #implementations

        #partial_declaration

        pub const model: #name<0> = #name::new();
      }
    };

    write!(f, "{output}")
  }
}
