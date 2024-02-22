use quote::__private::TokenStream;
use quote::quote;

use super::Identifier;

#[derive(Debug, Clone)]
pub enum Field {
  Property(FieldProperty),
  ForeignNode(FieldForeignNode),
  Relation(FieldRelation),
}

impl Field {
  pub fn emit_field(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_field(),
      Field::ForeignNode(x) => x.emit_field(),
      Field::Relation(x) => x.emit_field(),
    }
  }

  pub fn emit_initialization(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization(),
      Field::ForeignNode(x) => x.emit_initialization(),
      Field::Relation(x) => x.emit_initialization(),
    }
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization_with_origin(),
      Field::ForeignNode(x) => x.emit_initialization_with_origin(),
      Field::Relation(x) => x.emit_initialization_with_origin(),
    }
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_foreign_field_function(),
      Field::ForeignNode(x) => x.emit_foreign_field_function(),
      Field::Relation(x) => x.emit_foreign_field_function(),
    }
  }

  pub fn emit_partial_setter_field_function(&self) -> TokenStream {
    let field_name = match self {
      Field::Property(p) => &p.name,
      Field::ForeignNode(f) => &f.name,
      Field::Relation(r) => &r.name,
    };

    let name = field_name.to_ident();

    quote!(
      pub fn #name (mut self, value: impl serde::Serialize) -> Self {
        self.__insert_value_result(stringify!(#name), value)
      }
    )
  }
}

/// A simple property
#[derive(Debug, Clone)]
pub struct FieldProperty {
  pub name: Identifier,

  pub is_public: bool,
}

impl FieldProperty {
  fn emit_field(&self) -> TokenStream {
    let name = self.name.to_ident();
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #name: SchemaField<N>
    )
    .into()
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = self.name.to_ident();
    let name_str: &str = self.name.as_ref();

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = self.name.to_ident();
    let name_str: &str = self.name.as_ref();

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    quote!()
  }
}

/// A foreign node, like a foreign key that points to another `Model`
#[derive(Debug, Clone)]
pub struct FieldForeignNode {
  pub name: Identifier,
  pub foreign_type: Identifier,

  pub is_public: bool,
}

impl FieldForeignNode {
  fn emit_field(&self) -> TokenStream {
    let name = self.name.to_ident();
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #name: SchemaField<N>
    )
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = self.name.to_ident();
    let name_str: &str = self.name.as_ref();

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = self.name.to_ident();
    let name_str: &str = self.name.as_ref();

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let name = self.name.to_ident();
    let foreign_type = self.foreign_type.to_ident();

    quote!(
      pub fn #name (self) -> #foreign_type <{ N + 2 }> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_origin: [&'static str; N + 2] = [""; N + 2];
        new_origin[..N].clone_from_slice(&origin.segments);

        if (N > 0 && new_origin[N - 1] != ".") {
          new_origin[N] = ".";
        }

        new_origin[N + 1] = self.#name.identifier;

        #foreign_type::with_origin(OriginHolder::new(new_origin))
      }
    )
  }
}

/// A named relation
#[derive(Debug, Clone)]
pub struct FieldRelation {
  pub name: Identifier,
  pub foreign_type: Identifier,
  pub alias: Identifier,
  pub relation_type: FieldRelationType,
  pub is_public: bool,
}

#[derive(Debug, Clone)]
pub enum FieldRelationType {
  /// for `->` type of relations/edges
  OutgoingEdge,

  /// for `<-` type of relations/edges
  IncomingEdge,
}

impl FieldRelation {
  fn emit_field(&self) -> TokenStream {
    let alias = self.alias.to_ident();
    let attribute = match self.is_public {
      false => emit_skip_serializing_attribute(),
      true => quote!(),
    };

    quote!(
      #attribute
      pub #alias: SchemaField<N>
    )
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let alias = self.alias.to_ident();
    let name_str = format!("{}{}{}", self.name, self.edge(), self.foreign_type);
    let field_type = self.field_type();

    quote!(#alias: SchemaField::new(#name_str, #field_type))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let alias = self.alias.to_ident();
    let name_str = format!("{}{}{}", self.name, self.edge(), self.foreign_type);
    let field_type = self.field_type();

    quote!(#alias: SchemaField::with_origin(#name_str, #field_type, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let alias = self.alias.to_ident();
    let foreign_type = self.foreign_type.to_ident();
    let edge = self.edge();

    quote!(
      pub fn #alias (self) -> #foreign_type <{N + 2}> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_nested_origin: [&'static str; N + 2] = [""; N + 2];
        new_nested_origin[..N].clone_from_slice(&origin.segments);

        new_nested_origin[N] = #edge;
        new_nested_origin[N + 1] = self.#alias.identifier;

        #foreign_type::with_origin(OriginHolder::new(new_nested_origin))
      }
    )
  }

  fn edge(&self) -> &'static str {
    match &self.relation_type {
      FieldRelationType::OutgoingEdge => "->",
      FieldRelationType::IncomingEdge => "<-",
    }
  }

  fn field_type(&self) -> TokenStream {
    match &self.relation_type {
      FieldRelationType::OutgoingEdge => quote!(SchemaFieldType::Relation),
      FieldRelationType::IncomingEdge => quote!(SchemaFieldType::ForeignRelation),
    }
  }
}

fn emit_skip_serializing_attribute() -> TokenStream {
  quote!(#[serde(skip_serializing)])
}
