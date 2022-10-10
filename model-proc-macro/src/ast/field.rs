use quote::__private::TokenStream;
use quote::format_ident;
use quote::quote;

#[derive(Debug, Clone)]
pub enum Field {
  Property(FieldProperty),
  ForeignNode(FieldForeignNode),
  Relation(FieldRelation),
  ForeignRelation(FieldForeignRelation),
}

impl Field {
  pub fn emit_field(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_field(),
      Field::ForeignNode(x) => x.emit_field(),
      Field::Relation(x) => x.emit_field(),
      Field::ForeignRelation(x) => x.emit_field(),
    }
  }

  pub fn emit_initialization(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization(),
      Field::ForeignNode(x) => x.emit_initialization(),
      Field::Relation(x) => x.emit_initialization(),
      Field::ForeignRelation(x) => x.emit_initialization(),
    }
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_initialization_with_origin(),
      Field::ForeignNode(x) => x.emit_initialization_with_origin(),
      Field::Relation(x) => x.emit_initialization_with_origin(),
      Field::ForeignRelation(x) => x.emit_initialization_with_origin(),
    }
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    match self {
      Field::Property(x) => x.emit_foreign_field_function(),
      Field::ForeignNode(x) => x.emit_foreign_field_function(),
      Field::Relation(x) => x.emit_foreign_field_function(),
      Field::ForeignRelation(x) => x.emit_foreign_field_function(),
    }
  }
}

/// A simple property
#[derive(Debug, Clone)]
pub struct FieldProperty {
  pub name: String,
}

impl FieldProperty {
  fn emit_field(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);

    quote!(pub #name: SchemaField<N>).into()
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    quote!()
  }
}

/// A foreign node, like a foreign key that points to another `Model`
#[derive(Debug, Clone)]
pub struct FieldForeignNode {
  pub name: String,
  pub foreign_type: String,
}

impl FieldForeignNode {
  fn emit_field(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);

    quote!(pub #name: SchemaField<N>)
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Property))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Property, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let foreign_type = format_ident!("{}", self.foreign_type);

    quote!(
      pub fn #name (self) -> #foreign_type <{ N + 1 }> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_origin: [&'static str; N + 1] = [""; N + 1];
        new_origin[..N].clone_from_slice(&origin.segments);
        new_origin[N] = self.#name.identifier;

        #foreign_type::with_origin(OriginHolder::new(new_origin))
      }
    )
  }
}

/// A named relation
#[derive(Debug, Clone)]
pub struct FieldRelation {
  pub name: String,
  pub foreign_type: String,
}

impl FieldRelation {
  fn emit_field(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);

    quote!(pub #name: SchemaField<N>)
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::Relation))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::Relation, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;
    let foreign_type = format_ident!("{}", self.foreign_type);

    quote!(
      pub fn #name (self) -> RelationNode<#foreign_type <{ N + 1 }>> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_origin: [&'static str; N + 1] = [""; N + 1];
        new_origin[..N].clone_from_slice(&origin.segments);
        new_origin[N] = self.#name.identifier;

        RelationNode::new(
          #name_str,
          #foreign_type::with_origin(OriginHolder::new(new_origin))
        )
      }
    )
  }
}

/// A named relation that belongs to a foreign node
#[derive(Debug, Clone)]
pub struct FieldForeignRelation {
  pub name: String,
  pub foreign_type: String,
}

impl FieldForeignRelation {
  fn emit_field(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);

    quote!(pub #name: SchemaField<N>)
  }

  pub fn emit_initialization(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::new(#name_str, SchemaFieldType::ForeignRelation))
  }

  pub fn emit_initialization_with_origin(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;

    quote!(#name: SchemaField::with_origin(#name_str, SchemaFieldType::ForeignRelation, origin.clone()))
  }

  pub fn emit_foreign_field_function(&self) -> TokenStream {
    let name = format_ident!("{}", self.name);
    let name_str = &self.name;
    let foreign_type = format_ident!("{}", self.foreign_type);

    quote!(
      pub fn #name (self) -> RelationNode<#foreign_type <{ N + 1 }>> {
        let origin = self.origin.unwrap_or_else(|| OriginHolder::new([""; N]));
        let mut new_origin: [&'static str; N + 1] = [""; N + 1];
        new_origin[..N].clone_from_slice(&origin.segments);
        new_origin[N] = self.#name.identifier;

        RelationNode::new(
          #name_str,
          #foreign_type::with_origin(OriginHolder::new(new_origin))
        )
      }
    )
  }
}
