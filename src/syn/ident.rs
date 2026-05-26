use log::debug;
/// Get all [syn::Ident] from given [syn::Expr].
///
/// # Arguments
/// - `expr`— some [syn::Expr].
///
/// # Returns
/// A [Vec] of [syn::Ident].
pub fn get_idents_from_expr<'a>(expr: &'a syn::Expr) -> Vec<&'a syn::Ident> {
    let mut idents = vec![];

    match expr {
        // Main case: 'left ←op→ right'.
        syn::Expr::Binary(e) => {
            // Recurse into both sides of the op:
            idents.extend(&get_idents_from_expr(&e.left));
            idents.extend(&get_idents_from_expr(&e.right));
        },
        // A simple path with one segment is a plain id:
        syn::Expr::Path(e) => {
            if let Some(ident) = e.path.get_ident() {
                debug!("✅ Got ident '{}'", ident);
                idents.push(ident);
            }
        },
        // Casts, e.g. `X as Y`…
        syn::Expr::Cast(e) => {
            idents.extend(&get_idents_from_expr(&e.expr));
        },
        // …and that's all for now, folks.
        _ => {} 
    }

    idents
}

#[cfg(all(test, feature = "syn"))]
mod ident_tests {
    use super::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_get_idents_from_expr() {
       let expr: TokenStream = "pub const X: i32 = 5 * PI".parse().unwrap();
       debug!("expr = {:?}", expr);
    }
}
