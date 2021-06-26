use proc_macro::{TokenStream};
use proc_macro2::{Span};
use syn::{Data, DeriveInput, Expr, ExprLit, Lit, LitStr, Token, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn as_js_string(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    match &mut ast.data {
        Data::Enum(data_enum) => {
            for variant in &mut data_enum.variants {
                let value = variant.ident.to_string().to_lowercase();

                variant.discriminant = Some((Token![=](Span::call_site()), Expr::Lit(ExprLit {
                    attrs: vec![],
                    lit: Lit::Str(LitStr::new(&value, Span::call_site()))
                })))
            }
        },
        _ => todo!()
    }

    let gen = quote! { #ast };

    gen.into()
}