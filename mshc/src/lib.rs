//! MSHC-Core …
#[cfg(feature = "derive")]
pub use mshc_derive::*;
pub use mshc_macro_shared::*;
use quote::quote;
use syn::{Attribute, Data, DataEnum, DeriveInput, Fields, Ident};
pub mod named;


/// # Proc-macros
/// 
/// `attr` ident `what` is tagged as `goal`.
/// 
pub fn pm_is_tagged_attr(attr: &Attribute, what: &str, goal: &str) -> bool {
    if attr.path().is_ident(what) {
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(goal) {
                found = true;
            }
            Ok(())
        });
        return found;
    }

    false
}

/// # Proc-macros; `struct`-only
/// 
/// Get named `fields` of the given `input`.
/// 
pub fn pm_get_struct_fields(input: &DeriveInput) -> &syn::FieldsNamed {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields,
            _ => unimplemented!("Only named fields supported.")
        },

        _ => unimplemented!("Only structs supported.")
    }
}

pub fn pm_gen_container_match_method_to_field(data: &DataEnum, method: &Ident, num_arg: u32) -> Vec<proc_macro2::TokenStream> {
    pm_gen_container_match(data, method, method, num_arg)
}

pub fn pm_gen_container_match(data: &DataEnum, field: &Ident, method: &Ident, num_arg: u32) -> Vec<proc_macro2::TokenStream> {
    data.variants.iter().map(|variant| {
        let arg = match num_arg {
            0 => quote!(),
            1 => quote!(a),
            2 => quote!(a,b),
            3 => quote!(a,b,c),
            _ => quote!(a,b,c,d),
        };
        let var_ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(_) => {
                quote! {
                    Self::#var_ident(inner) => inner.#method(#arg)
                }
            }

            Fields::Named(_) => {
                quote! {
                    Self::#var_ident { #field, ..} => #field.#method(#arg)
                }
            }

            Fields::Unit => { quote! { Self::#var_ident => panic!("No… Units are too weird stuff! Derive manually.") }}
        }
    }).collect()
}

pub fn pm_gen_container_match_as_method_or_direct_bool_field(
    data: &DataEnum,
    field: &Ident,
    method: &Ident,
    num_arg: u32
) -> Vec<proc_macro2::TokenStream> {
    data.variants.iter().map(|variant| {
        let arg = match num_arg {
            0 => quote!(),
            1 => quote!(a),
            2 => quote!(a,b),
            3 => quote!(a,b,c),
            _ => quote!(a,b,c,d),
        };
        let var_ident = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(_) => {
                quote! {
                    Self::#var_ident(inner) => inner.#method(#arg)
                }
            }

            Fields::Named(named_fs) => {
                match maybe_field!(named named_fs, field) {
                    Some(fld) => quote! { Self::#var_ident { #fld, ..} => *#fld },
                    _ => quote! { Self::#var_ident {..} => false }
                }
            }

            Fields::Unit => { quote! { Self::#var_ident => false }}
        }
    }).collect()
}

#[macro_export]
macro_rules! get_tagged_ident {
    ($data:ident, $tag:literal, $name:literal) => {
        $data.fields.iter().find(|f| {
            f.attrs.iter().any(|attr| $crate::pm_is_tagged_attr(attr, $tag, $name)) ||
            f.ident.as_ref().map_or(false, |i| i == $name)
        })  .map(|f| f.ident.as_ref().unwrap())
            .expect(&format!("Field/tag '{}' not found", $name))
    };
}
