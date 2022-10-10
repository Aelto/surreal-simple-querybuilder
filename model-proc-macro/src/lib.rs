use std::str::FromStr;

use lalrpop_util::lalrpop_mod;
use proc_macro::TokenStream;

mod ast;

lalrpop_mod!(parser);

#[proc_macro]
pub fn model(input: TokenStream) -> TokenStream {
  let content = input.to_string();
  let model = parser::ModelParser::new().parse(&content).unwrap();

  let output = model.to_string();
  TokenStream::from_str(&output).unwrap()
}
