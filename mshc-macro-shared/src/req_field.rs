#[macro_export]
macro_rules! req_field {
    (named $data:ident, $field:literal) => {
        $data.named.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
            .expect(&format!("No '{}' field found in struct layout", $field))
    };

    ($data:ident, $field:literal) => {
        $data.fields.iter().find(|f| {
            f.ident.as_ref().map_or(false, |i| i == $field)
        })  .map(|f| f.ident.as_ref().unwrap())
            .expect(&format!("No '{}' field found in struct layout", $field))
    };
}
