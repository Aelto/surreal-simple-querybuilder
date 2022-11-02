use std::fmt::Debug;
use std::fmt::Display;

use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;

use super::Field;

#[derive(Debug)]
pub struct Model {
  pub name: String,
  pub fields: Vec<Field>,
}

impl Display for Model {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format_ident!("{}", self.name);

    let field_declarations: Vec<TokenStream> =
      self.fields.iter().map(|field| field.emit_field()).collect();

    let struct_declaration = quote! {
      pub struct #name <const N: usize> {
        origin: Option<OriginHolder<N>>,
        #(#field_declarations),*
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

        pub fn do_label(self,id: &str) -> #name<{N+2}> {
          let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
          let mut new_origin: [&'static str; N + 2] = [""; N + 2];
          new_origin[..N].clone_from_slice(&origin.segments);

          if (N > 0 && new_origin[N - 1] != ":") {
            new_origin[N] = ":";
          }

          new_origin[N + 1] = Box::leak(id.to_owned().into_boxed_str());/// todo

          #name::with_origin(OriginHolder::new(new_origin))
        }

        #(#field_foreign_functions)*
      }

      impl<const N: usize> std::fmt::Display for #name<N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{}", Self::label)
        }
      }

      impl<const N: usize> IntoQueryBuilderSegment for #name<N> {
        fn into<'b>(self, querybuilder: &mut QueryBuilder<'b>) -> QueryBuilderSegment<'b>
        where
          Self: 'b,
        {
          querybuilder.hold(self.to_string())
        }
      }

      impl<const N: usize> ToNodeBuilder for #name<N> {}
    };

    let output = quote! {
      pub mod schema {
        use super::*;
        use surreal_simple_querybuilder::prelude::*;

        #struct_declaration
        #implementations

        pub const model: #name<0> = #name::new();
      }
    };

    write!(f, "{output}")
  }
}
