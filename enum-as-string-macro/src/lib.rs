use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};
use proc_macro_error::emit_call_site_error;
use syn::{Data, DeriveInput, Expr, ExprLit, Lit, LitStr, Token, parse::{Parse, ParseStream}, parse_macro_input};
use quote::quote;

#[derive(Default)]
struct RootAttributes {
    discriminant: bool,
    lowercase: bool
}

impl Parse for RootAttributes {
    fn parse(content: ParseStream) -> syn::Result<Self> {
        let mut attributes = RootAttributes::default();

        while !content.is_empty() {
            let name = content.parse::<Ident>()?.to_string();

            match name.as_str() {
                "discriminant" => attributes.discriminant = true,
                "lowercase" => attributes.lowercase = true,
                _ => {}
            }

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(attributes)
    }
}

#[proc_macro_attribute]
pub fn enum_as_string(attr: TokenStream, input: TokenStream) -> TokenStream {
    let root_attributes = match syn::parse::<RootAttributes>(attr.clone()) {
        Ok(attributes) => attributes,
        Err(error) => {
            emit_call_site_error!(error);
            RootAttributes::default()
        }
    };

    let mut ast = parse_macro_input!(input as DeriveInput);
    let mut from_lines = vec![];
    let mut to_lines = vec![];

    match &mut ast.data {
        Data::Enum(data_enum) => {
            for variant in &mut data_enum.variants {
                let ident = &variant.ident;
                let mut value = ident.to_string();

                if root_attributes.lowercase {
                    value = value.to_lowercase();
                }

                let lit = Lit::Str(LitStr::new(&value, Span::call_site()));

                from_lines.push(quote! { #lit => Self::#ident });
                to_lines.push(quote! { Self::#ident => #lit });
                
                if root_attributes.discriminant {
                    variant.discriminant = Some((Token![=](Span::call_site()), Expr::Lit(ExprLit {
                        attrs: vec![],
                        lit
                    })))
                }
            }
        },
        _ => todo!()
    }

    let enum_name = &ast.ident;
    let gen = quote! {
        #ast

        impl ToString for #enum_name {
            fn to_string(&self) -> String {
                match self {
                    #(#to_lines),*,
                    _ => ""
                }.to_string()
            }
        }

        impl std::str::FromStr for #enum_name {
            type Err = ();

            fn from_str(string: &str) -> Result<Self, Self::Err> {
                Ok(match string {
                    #(#from_lines),*,
                    _ => unreachable!()
                })
            }
        }
    };

    gen.into()
}