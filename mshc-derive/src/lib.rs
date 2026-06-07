//! Proc-macro stuff.

use syn::{Data, DeriveInput};

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
