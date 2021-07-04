#![allow(unused_assignments)]

use proc_macro::{TokenStream};
use quote::quote;
use syn::{*, parse::{Parse, ParseStream}};
use proc_macro_error::*;

#[derive(Default)]
struct ParsableAttributes {
    regex: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    min: Option<usize>,
    sep: Option<String>,
    optional: Option<bool>,
}

impl Parse for ParsableAttributes {
    #[allow(unused_must_use)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = ParsableAttributes::default();
        let content;

        parenthesized!(content in input);

        while !content.is_empty() {
            let name = content.parse::<Ident>()?.to_string();

            content.parse::<Token![=]>()?;

            match name.as_str() {
                "regex" => attributes.regex = Some(content.parse::<LitStr>()?.value()),
                "prefix" => attributes.prefix = Some(content.parse::<LitStr>()?.value()),
                "suffix" => attributes.suffix = Some(content.parse::<LitStr>()?.value()),
                "brackets" => {
                    let brackets = content.parse::<LitStr>()?.value();

                    if brackets.len() == 2 {
                        attributes.prefix = Some((brackets.as_bytes()[0] as char).to_string());
                        attributes.suffix = Some((brackets.as_bytes()[1] as char).to_string());
                    }
                },
                "min" => attributes.min = Some(content.parse::<LitInt>()?.base10_parse::<usize>()?),
                "sep" => attributes.sep = Some(content.parse::<LitStr>()?.value()),
                "optional" => attributes.optional = Some(content.parse::<LitBool>()?.value()),
                _ => {}
            }

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(attributes)
    }
}

// https://docs.rs/syn/1.0.73/syn/struct.DeriveInput.html
#[proc_macro_error]
#[proc_macro_attribute]
pub fn parsable(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast : DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let body = match &mut ast.data {
        Data::Struct(data_struct) => {
            match &mut data_struct.fields {
                Fields::Named(named_fields) => {
                    let mut field_names = vec![];
                    let mut lines = vec![];

                    for field in named_fields.named.iter_mut() {
                        let mut attributes = ParsableAttributes::default();
                        let field_name = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;

                        field_names.push(quote! { #field_name });

                        if let Some((i, attr)) = field.attrs.iter().enumerate().find(|(_, attr)| attr.path.segments.last().unwrap().ident == "parsable") {
                            let result = syn::parse2::<ParsableAttributes>(attr.tokens.clone());

                            match result {
                                Ok(value) => attributes = value,
                                Err(error) => emit_call_site_error!(error)
                            };

                            field.attrs.remove(i);
                        }

                        let is_vec = match field_type {
                            Type::Path(type_path) => type_path.path.segments.last().unwrap().ident == "Vec",
                            _ => false,
                        };
                        let is_option = match field_type {
                            Type::Path(type_path) => type_path.path.segments.last().unwrap().ident == "Option",
                            _ => false,
                        };

                        let optional = is_option || attributes.optional.map_or(false, |value| value);
                        let on_success = quote! { reader__.eat_spaces(); };
                        let mut on_fail = quote ! {
                            reader__.set_index(start_index__);
                            return None;
                        };

                        if optional {
                            on_fail = quote! {
                                field_failed__ = true;
                                reader__.set_index(field_index__);
                                <#field_type as Default>::default()
                            };
                        }

                        let mut check = vec![];
                        let has_prefix = attributes.prefix.is_some();
                        let prefix_parsing = match attributes.prefix {
                            Some(prefix) => quote! {
                                match reader__.read_string(#prefix) {
                                    Some(_) => reader__.eat_spaces(),
                                    None => {
                                        reader__.set_expected_token(format!("{:?}", #prefix));
                                        prefix_ok__ = false;
                                        #on_fail;
                                    }
                                };
                            },
                            None => quote! {}
                        };
                        let suffix_parsing = match attributes.suffix {
                            Some(suffix) => quote! {
                                match reader__.read_string(#suffix) {
                                    Some(_) => reader__.eat_spaces(),
                                    None => {
                                        reader__.set_expected_token(format!("{:?}", #suffix));
                                        #on_fail;
                                    }
                                };
                            },
                            None => quote! {}
                        };

                        let mut parse_method = quote! { parse(reader__) };

                        if let Some(separator) = attributes.sep {
                            parse_method = quote! { parse_with_separator(reader__, #separator) };
                        }

                        let mut assignment = quote! {
                            let #field_name = match <#field_type as lotus_parsable::Parsable>::#parse_method {
                                Some(value) => value,
                                None => {
                                    reader__.set_expected_token(<#field_type as lotus_parsable::Parsable>::get_token_name());
                                    #on_fail
                                }
                            };
                        };

                        if has_prefix && optional {
                            assignment = quote! {
                                let #field_name = match prefix_ok__ {
                                    true => match <#field_type as lotus_parsable::Parsable>::#parse_method {
                                        Some(value) => value,
                                        None => {
                                            reader__.set_expected_token(<#field_type as lotus_parsable::Parsable>::get_token_name());
                                            #on_fail
                                        }
                                    },
                                    false => <#field_type as Default>::default()
                                };
                            };

                            // assignment = quote! {
                            //     let #field_name = <#field_type as Default>::default();
                            // };
                        }

                        if let Some(pattern) = attributes.regex {
                            assignment = quote! {
                                let #field_name = match reader__.read_regex(#pattern) {
                                    Some(value) => match <#field_type as std::str::FromStr>::from_str(value) {
                                        Ok(value) => value,
                                        Err(_) => { #on_fail }
                                    },
                                    None => { #on_fail }
                                };
                            }
                        }

                        if let Some(min) = attributes.min {
                            check.push(quote! {
                                if !field_failed__ && #field_name.len() < #min {
                                    reader__.set_expected_token(<#field_type as lotus_parsable::Parsable>::get_token_name());
                                    #on_fail;
                                }
                            });
                        }

                        if is_option && has_prefix {
                            check.push(quote! {
                                if #field_name.is_none() {
                                    #on_fail;
                                }
                            });
                        }

                        if is_vec && has_prefix {
                            check.push(quote! {
                                if #field_name.is_empty() && prefix_ok__ {
                                    reader__.set_expected_token(<#field_type as lotus_parsable::Parsable>::get_token_name());
                                    #on_fail;
                                }
                            });
                        }

                        lines.push(quote! {
                            field_failed__ = false;
                            field_index__ = reader__.get_index();
                            prefix_ok__ = true;
                            #prefix_parsing
                            #assignment
                            #(#check)*
                            #suffix_parsing
                            #on_success
                        });
                    }

                    quote! {
                        let mut field_index__ : usize = 0;
                        let mut field_failed__ = false;
                        let mut prefix_ok__ = true;
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
                let variant = &mut data_enum.variants[i];
                let variant_name = &variant.ident;
                let mut attributes = ParsableAttributes::default();
                let mut parse_prefix = quote! { true };
                let mut parse_suffix = quote! { true };
                let mut parse_method = quote! { parse(reader__) };

                if let Some((i, attr)) = variant.attrs.iter().enumerate().find(|(_, attr)| attr.path.segments.last().unwrap().ident == "parsable") {
                    let result = syn::parse2::<ParsableAttributes>(attr.tokens.clone());

                    match result {
                        Ok(value) => attributes = value,
                        Err(error) => emit_call_site_error!(error)
                    };

                    variant.attrs.remove(i);
                }

                if let Some(prefix) = attributes.prefix {
                    parse_prefix = quote! {
                        match reader__.read_string(#prefix) {
                            Some(_) => { reader__.eat_spaces(); true },
                            None => { reader__.set_expected_token(format!("{:?}", #prefix)); false }
                        }
                    };
                }

                if let Some(suffix) = attributes.suffix {
                    parse_suffix = quote! {
                        match reader__.read_string(#suffix) {
                            Some(_) => { reader__.eat_spaces(); true },
                            None => { reader__.set_expected_token(format!("{:?}", #suffix)); false }
                        }
                    };
                }

                if let Some(separator) = attributes.sep {
                    parse_method = quote! { parse_with_separator(reader__, #separator) };
                }

                match &variant.fields {
                    Fields::Named(_fields_named) => todo!(),
                    Fields::Unnamed(fields_unnamed) => {
                        let field = &fields_unnamed.unnamed[0];
                        let field_type = &field.ty;

                        lines.push(quote! {
                            let prefix_ok__ = #parse_prefix;

                            if prefix_ok__ {
                                if let Some(value) = <#field_type as lotus_parsable::Parsable>::#parse_method {
                                    reader__.eat_spaces();

                                    let suffix_ok__ = #parse_suffix;

                                    if suffix_ok__ {
                                        return Some(Self::#variant_name(value))
                                    }
                                }
                            }

                            reader__.set_index(start_index__);
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
                                    if let Some(_) = reader__.read_string(#lit_str) {
                                        reader__.eat_spaces();
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

                // reader__.set_expected_token(<Self as lotus_parsable::Parsable>::get_token_name());
                None
            }
        },
        _ => todo!()
    };

    let result = quote! {
        #ast

        impl lotus_parsable::Parsable for #name {
            fn parse(reader__: &mut lotus_parsable::StringReader) -> Option<Self> {
                let start_index__ = reader__.get_index();

                #body
            }
        }
    };

    result.into()
}