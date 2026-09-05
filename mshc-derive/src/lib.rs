//! Proc-macro stuff.
use mshc_macro_shared::{structs::get_struct_fields, *};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derive [Named] trait.
#[proc_macro_derive(Named)]
pub fn mshc_derive_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = get_struct_fields(&input);
    let name_f = req_ident!(named fields, "name");
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
    let name_f = req_ident!(named fields, "name");
    TokenStream::from(quote! {
        impl mshc::named::NamedMut for #name {
            fn set_name(&mut self, name: &str) -> &mut Self {
                self.#name_f = name.into();
                self
            }
        }
    })
}

/// Derive [Graded] trait.
#[proc_macro_derive(Graded)]
pub fn mshc_derive_graded(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = get_struct_fields(&input);
    let grade_f = req_ident!(named fields, "grade");
    TokenStream::from(quote! {
        impl mshc::grade::Graded for #name {
            fn grade(&self) -> mshc::grade::Grade {
                self.#grade_f
            }
        }
    })
}

/// Derive [GradeMut] trait.
#[proc_macro_derive(GradeMut)]
pub fn mshc_derive_grademut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = get_struct_fields(&input);
    let grade_f = req_ident!(named fields, "grade");
    TokenStream::from(quote! {
        impl mshc::grade::GradeMut for #name {
            fn set_grade(&mut self, grade: Grade) {
                self.#grade_f = grade
            }
        }
    })
}
