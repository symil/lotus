#![allow(unused_assignments)]

use proc_macro::{TokenStream};
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn macro_derive_parsable(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast : DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let body = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named_fields) => {
                    let lines : Vec<proc_macro2::TokenStream> = named_fields.named.iter().map(|field| {
                        let name = &field.ident.as_ref().unwrap();
                        let ty = &field.ty;

                        quote! {
                            #name: match <#ty>::parse(entry)
                        }
                    }).collect();

                    quote! {
                        Self {
                            #(#lines),*
                        }
                    }
                },
                Fields::Unnamed(_fields) => todo!(),
                Fields::Unit => todo!()
            }
        },
        Data::Enum(data_enum) => {
            let mut lines = vec![];

            for i in 0..data_enum.variants.len() {
                let variant = &data_enum.variants[i];
                let name = &variant.ident;
                let rule_ident = &variant.attrs[0].path; // TODO: check if it exists

                match &variant.fields {
                    Fields::Named(_fields_named) => todo!(),
                    Fields::Unnamed(fields_unnamed) => {
                        let field = &fields_unnamed.unnamed[0];
                        let ty = &field.ty;

                        lines.push(quote! {
                            crate::grammar::Rule::#rule_ident => Self::#name(<#ty>::parse(entry))
                        });
                    },
                    Fields::Unit => {
                        lines.push(quote! {
                            crate::grammar::Rule::#rule_ident => Self::#name
                        });
                    }
                }
            }

            quote! {
                match entry.as_rule() {
                    #(#lines),*,
                    _ => unreachable!()
                }
            }
        },
        _ => todo!()
    };

    let result = quote! {
        impl crate::grammar::Parsable for #name {
            fn parse(entry: pest::iterators::Pair<crate::grammar::Rule>) -> Self {
                #body
            }
        }
    };

    result.into()
}