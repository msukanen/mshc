#[macro_export]
macro_rules! req_ident {
    (named $data:ident, $field:expr) => {
        $crate::maybe_ident!(named $data, $field)
            .expect(&format!("No '{}' field found in struct layout", $field))
    };

    ($data:ident, $field:expr) => {
        $crate::maybe_ident!($data, $field)
            .expect(&format!("No '{}' field found in struct layout", $field))
    };
}

#[macro_export]
macro_rules! req_field {
    (named $data:ident, $field:expr) => {
        $crate::maybe_field!(named $data, $field)
            .expect(&format!("No '{}' field found in struct layout", $field))
    };

    ($data:ident, $field:expr) => {
        $crate::maybe_field!($data, $field)
            .expect(&format!("No '{}' field found in struct layout", $field))
    };
}
