#![allow(unused_assignments)]

use proc_macro::{TokenStream};
use quote::quote;
use syn::{self, Data, Fields, GenericParam, Ident, TypeParamBound};

#[proc_macro_derive(Serializable)]
pub fn serializable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_serializable_macro(&ast)
}

// https://docs.rs/syn/1.0.73/syn/struct.DeriveInput.html
fn impl_serializable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut serialize = quote! { };
    let mut deserialize = quote! { None };

    match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named_fields) => {
                    let mut serialize_lines = vec![];
                    let mut deserialize_lines = vec![];

                    for field in &named_fields.named {
                        let name = &field.ident.as_ref().unwrap();
                        let ty = &field.ty;

                        serialize_lines.push(quote! {
                            <#ty>::write_bytes(&value.#name, bytes);
                        });

                        deserialize_lines.push(quote! {
                            #name: match <#ty>::read_bytes(buffer) {
                                None => return None,
                                Some(value) => value,
                            }
                        });
                    }

                    serialize = quote! { #(#serialize_lines)* };
                    deserialize = quote! {
                        Some(Self {
                            #(#deserialize_lines),*
                        })
                    }
                },
                Fields::Unnamed(_fields) => todo!(),
                Fields::Unit => todo!()
            }
        },
        Data::Enum(data_enum) => {
            // TODO: change encoding size depending on the number of fields in the enum
            let mut serialize_lines = vec![];
            let mut deserialize_lines = vec![];

            for i in 0..data_enum.variants.len() {
                let n = i as u8;
                let variant = &data_enum.variants[i];
                let name = &variant.ident;

                match &variant.fields {
                    Fields::Named(_fields_named) => todo!(),
                    Fields::Unnamed(fields_unnamed) => {
                        let mut sub_vars = vec![];
                        let mut sub_serialize_lines = vec![];
                        let mut sub_deserialize_lines = vec![];
                        let mut j : usize = 0;

                        for field in &fields_unnamed.unnamed {
                            let ty = &field.ty;
                            let name = Ident::new(&format!("v_{}", &j), fields_unnamed.paren_token.span); // didn't manage to create a new span, this will do for now
                            
                            sub_vars.push(quote! { #name });
                            sub_serialize_lines.push(quote! { <#ty>::write_bytes(#name, bytes); });
                            sub_deserialize_lines.push(quote! {
                                let #name = match <#ty>::read_bytes(buffer) {
                                    None => return None,
                                    Some(value) => value
                                };
                            });

                            j += 1;
                        }

                        serialize_lines.push(quote! {
                            Self::#name(#(#sub_vars),*) => {
                                bytes.push(#n);
                                #(#sub_serialize_lines)*
                            }
                        });

                        deserialize_lines.push(quote! {
                            #n => {
                                #(#sub_deserialize_lines)*
                                Self::#name(#(#sub_vars),*)
                            }
                        });
                    },
                    Fields::Unit => {
                        serialize_lines.push(quote! {
                            Self::#name => {
                                bytes.push(#n);
                            }
                        });

                        deserialize_lines.push(quote! {
                            #n => Self::#name
                        });
                    }
                }
            }

            serialize = quote! {
                match value {
                    #(#serialize_lines),*
                }
            };

            deserialize = quote! {
                match u8::read_bytes(buffer) {
                    None => return None,
                    Some(header) => Some(match header {
                        #(#deserialize_lines),*,
                        _ => return None
                    })
                }
            };
        },
        Data::Union(_data_union) => todo!()
    };

    let mut generics_name_list = vec![];
    let mut generics_def_list = vec![];
    let mut generics_type = quote! {};
    let mut generics_impl = quote! {};

    for param in &ast.generics.params {
        match param {
            GenericParam::Type(type_param) => {
                let name = &type_param.ident;
                let mut paths = vec![];

                generics_name_list.push(quote! { #name });

                for param in &type_param.bounds {
                    match param {
                        TypeParamBound::Trait(trait_bound) => {
                            paths.push(&trait_bound.path);
                        },
                        TypeParamBound::Lifetime(_) => todo!(),
                    }
                }

                if paths.len() > 0 {
                    generics_def_list.push(quote! { #name : #(#paths)+* });
                } else {
                    generics_def_list.push(quote! { #name });
                }
            },
            GenericParam::Lifetime(_) => todo!(),
            GenericParam::Const(_) => todo!(),
        }
    }

    if generics_name_list.len() > 0 {
        generics_type = quote! { <#(#generics_name_list),*> };
        generics_impl = quote! { <#(#generics_def_list),*> };
    }

    let gen = quote! {
        impl#generics_impl Serializable for #name#generics_type {
            fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
                #serialize
            }

            fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
                #deserialize
            }
        }
    };

    gen.into()
}