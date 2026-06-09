//! Proc-macro stuff.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Required field, either from input.data or input.data.fields
macro_rules! req_field {
    (named $data:ident, $field:literal) => {
        $data.named.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
            .expect(&format!("No '{}' field found in #name", $field))
    };

    ($data:ident, $field:literal) => {
        $data.fields.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
            .expect(&format!("No '{}' field found in #name", $field))
    };
}

macro_rules! maybe_field {
    (named $data:ident, $field:literal) => {
        $data.named.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
    };

    ($data:ident, $field:literal) => {
        $data.fields.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
    };
}

fn get_struct_fields(input: &DeriveInput) -> &syn::FieldsNamed {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields,
            _ => unimplemented!("Only named fields supported.")
        },

        _ => unimplemented!("Only structs supported.")
    }
}

/// Derive [Named] trait.
#[proc_macro_derive(Named)]
pub fn mshc_derive_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = get_struct_fields(&input);
    let name_f = req_field!(named fields, "name");
    TokenStream::from(quote! {
        impl mshc::named::Named for #name {
            fn name<'a>(&'a self) -> &'a str {
                &self.#name_f
            }
        }
    })
}

/// Derive [NamedMut] trait.
#[proc_macro_derive(NamedMut)]
pub fn mshc_derive_named_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = get_struct_fields(&input);
    let name_f = req_field!(named fields, "name");
    TokenStream::from(quote! {
        impl mshc::named::NamedMut for #name {
            fn set_name(&mut self, name: &str) -> &mut Self {
                self.#name_f = name.into();
                self
            }
        }
    })
}
