use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Field;
use syn::Ident;

pub mod options;

pub fn emit_querybuilder_object_impl(type_name: &Ident, fields: &syn::Fields, model_name: &str) -> TokenStream {
  let name = format_ident!("{}", type_name);
  let model_name = format_ident!("{}", model_name);
  let field_declarations: Vec<TokenStream> = fields
    .iter()
    .map(|field| emit_field(field, &model_name))
    .collect();

    let first_field = field_declarations.iter().take(1);
    let other_fields = field_declarations.iter().skip(1)
      .map(|f| quote! {
        .also(#f)
      });

  quote! {
    impl QueryBuilderObject for #name {
      fn set_querybuilder_object<'a>(querybuilder: QueryBuilder<'a>) -> QueryBuilder {
        querybuilder.set(#(#first_field),*)
          #(#other_fields)*
      }
    }
  }
}

fn emit_field(field: &Field, model_name: &Ident) -> TokenStream {
  let field = field.ident.as_ref().unwrap().to_string();
  let field = format_ident!("{}", field);

  quote! {
    #model_name.#field.equals_parameterized()
  }.into()
}