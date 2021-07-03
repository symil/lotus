#![allow(unused_assignments)]

use proc_macro::{TokenStream};
use quote::quote;
use syn::*;
use proc_macro_error::*;

// https://docs.rs/syn/1.0.73/syn/struct.DeriveInput.html
#[proc_macro_error]
#[proc_macro_attribute]
pub fn parsable(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast : DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let body = match &mut ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named_fields) => {
                    let mut field_names = vec![];
                    let mut lines = vec![];

                    for field in named_fields.named.iter() {
                        let field_name = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;

                        field_names.push(field_name);
                        lines.push(quote! {
                            let #field_name = match <#field_type as lotus_parsable::Parsable>::parse(reader) {
                                Some(value__) => value__,
                                None => {
                                    reader.set_index(start_index__);
                                    reader.set_error::<#field_type>();
                                    return None;
                                }
                            };
                            reader.eat_spaces();
                        });
                    }

                    quote! {
                        #(#lines)*
                        Some(Self { #(#field_names),* })
                    }
                },
                Fields::Unnamed(_) => todo!(),
                Fields::Unit => todo!()
            }
        },
        Data::Enum(data_enum) => {
            let mut lines = vec![];

            for i in 0..data_enum.variants.len() {
                let variant = &data_enum.variants[i];
                let variant_name = &variant.ident;

                match &variant.fields {
                    Fields::Named(_fields_named) => todo!(),
                    Fields::Unnamed(fields_unnamed) => {
                        let field = &fields_unnamed.unnamed[0];
                        let field_type = &field.ty;

                        lines.push(quote! {
                            if let Some(value) = <#field_type as lotus_parsable::Parsable>::parse(reader) {
                                reader.eat_spaces();
                                return Some(Self::#variant_name(value))
                            }
                        });
                    },
                    Fields::Unit => {
                        let string = match &variant.discriminant {
                            Some((_, Expr::Lit(expr_lit))) => {
                                match &expr_lit.lit {
                                    Lit::Str(value) => {
                                        Some(value)
                                    },
                                    _ => None
                                }
                            },
                            _ => None
                        };

                        match string {
                            Some(lit_str) => {
                                lines.push(quote! {
                                    if let Some(_) = reader.read_string(#lit_str) {
                                        reader.eat_spaces();
                                        return Some(Self::#variant_name);
                                    }
                                });
                            },
                            None => emit_call_site_error!("variants with no field must have an associated string literal")
                        }
                    }
                }
            }

            for variant in data_enum.variants.iter_mut() {
                variant.discriminant = None;
            }

            quote! {
                #(#lines)*

                reader.set_error::<Self>();
                None
            }
        },
        _ => todo!()
    };

    let result = quote! {
        #ast

        impl lotus_parsable::Parsable for #name {
            fn parse(reader: &mut lotus_parsable::StringReader) -> Option<Self> {
                let start_index__ = reader.get_index();

                #body
            }
        }
    };

    result.into()
}