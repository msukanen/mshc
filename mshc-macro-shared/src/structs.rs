/// Get (named) struct fields.
pub fn get_struct_fields(input: &syn::DeriveInput) -> &syn::FieldsNamed {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields,
            _ => unimplemented!("Only named fields supported.")
        },

        _ => unimplemented!("Only structs supported.")
    }
}
