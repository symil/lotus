#![allow(unused_assignments)]

use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::quote;
use syn::{*, parse::{Parse, ParseStream}};
use proc_macro_error::*;

#[derive(Default)]
struct FieldAttributes {
    regex: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    min: Option<usize>,
    separator: Option<String>,
    optional: Option<bool>,
    ignore: bool
}

impl Parse for FieldAttributes {
    #[allow(unused_must_use)]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = FieldAttributes::default();
        let content;

        parenthesized!(content in input);

        while !content.is_empty() {
            let name = content.parse::<Ident>()?.to_string();

            if name.as_str() == "ignore" {
                attributes.ignore = true;
            } else {
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
                    "sep" => attributes.separator = Some(content.parse::<LitStr>()?.value()),
                    "separator" => attributes.separator = Some(content.parse::<LitStr>()?.value()),
                    "optional" => attributes.optional = Some(content.parse::<LitBool>()?.value()),
                    _ => {}
                }
            }

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(attributes)
    }
}

struct RootAttributes {
    located: bool,
    name: Option<String>
}

impl Default for RootAttributes {
    fn default() -> Self {
        Self { located: true, name: None }
    }
}

impl Parse for RootAttributes {
    fn parse(content: ParseStream) -> syn::Result<Self> {
        let mut attributes = RootAttributes::default();

        while !content.is_empty() {
            let name = content.parse::<Ident>()?.to_string();
            content.parse::<Token![=]>()?;

            match name.as_str() {
                "located" => attributes.located = content.parse::<LitBool>()?.value(),
                "name" => attributes.name = Some(content.parse::<LitStr>()?.value()),
                _ => {}
            }

            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(attributes)
    }
}

fn is_type(ty: &Type, name: &str) -> bool {
    get_type_name(ty) == name
}

fn get_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
        _ => todo!(),
    }
}

struct Wrapper {
    field: Field
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let field = Field::parse_named(input)?;

        Ok(Self { field })
    }
}

fn create_location_field(field_name: &str) -> Field {
    let string = format!("pub {}: parsable::DataLocation", field_name);
    let result : Result<Wrapper> = syn::parse_str(&string);

    result.unwrap().field
}

// https://docs.rs/syn/1.0.74/syn/struct.DeriveInput.html
#[proc_macro_error]
#[proc_macro_attribute]
pub fn parsable(attr: TokenStream, input: TokenStream) -> TokenStream {
    let root_attributes = match syn::parse::<RootAttributes>(attr.clone()) {
        Ok(attributes) => attributes,
        Err(error) => {
            emit_call_site_error!(error);
            RootAttributes::default()
        }
    };
    let located = root_attributes.located;
    let mut ast : DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let derive_attribute = syn::Attribute {
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
        bracket_token: syn::token::Bracket { span: Span::call_site() },
        path: syn::parse_str("derive").unwrap(),
        tokens: syn::parse_str("(Debug, Clone)").unwrap(),
    };

    // if let Data::Enum(_) = ast.data {
    //     derive_attribute.tokens = syn::parse_str("(Debug, Clone, Copy, PartialEq)").unwrap();
    // }

    ast.attrs.push(derive_attribute);

    let mut impl_get_location = quote! { };

    let body = match &mut ast.data {
        Data::Struct(data_struct) => {
            impl_get_location = quote! {
                fn get_location(&self) -> &parsable::DataLocation {
                    &self.location
                }
            };

            match &mut data_struct.fields {
                Fields::Named(named_fields) => {
                    let mut field_names = vec![];
                    let mut lines = vec![];

                    for field in named_fields.named.iter_mut() {
                        let mut attributes = FieldAttributes::default();
                        let is_vec = is_type(&field.ty, "Vec");
                        let is_option = is_type(&field.ty, "Option");

                        let field_name = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;

                        field_names.push(quote! { #field_name });

                        if let Some((i, attr)) = field.attrs.iter().enumerate().find(|(_, attr)| attr.path.segments.last().unwrap().ident == "parsable") {
                            let result = syn::parse2::<FieldAttributes>(attr.tokens.clone());

                            match result {
                                Ok(value) => attributes = value,
                                Err(error) => emit_call_site_error!(error)
                            };

                            field.attrs.remove(i);
                        }

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
                        let has_suffix = attributes.suffix.is_some();
                        let prefix_parsing = match attributes.prefix {
                            Some(prefix) => quote! {
                                match reader__.read_string(#prefix) {
                                    Some(_) => reader__.eat_spaces(),
                                    None => {
                                        reader__.set_expected_token(Some(format!("{:?}", #prefix)));
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
                                        reader__.set_expected_token(Some(format!("{:?}", #suffix)));
                                        #on_fail;
                                    }
                                };
                            },
                            None => quote! {}
                        };

                        let mut parse_method = quote! { parse(reader__) };

                        if let Some(separator) = attributes.separator {
                            parse_method = quote! { parse_with_separator(reader__, #separator) };
                        }

                        let mut assignment = quote! {
                            let #field_name = match <#field_type as parsable::Parsable>::#parse_method {
                                Some(value) => value,
                                None => {
                                    reader__.set_expected_token(<#field_type as parsable::Parsable>::get_token_name());
                                    #on_fail
                                }
                            };
                        };

                        if has_prefix && optional {
                            assignment = quote! {
                                let #field_name = match prefix_ok__ {
                                    true => match <#field_type as parsable::Parsable>::#parse_method {
                                        Some(value) => value,
                                        None => {
                                            reader__.set_expected_token(<#field_type as parsable::Parsable>::get_token_name());
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
                                    reader__.set_expected_token(<#field_type as parsable::Parsable>::get_token_name());
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

                        if is_vec && has_prefix && !has_suffix {
                            check.push(quote! {
                                if #field_name.is_empty() && prefix_ok__ {
                                    reader__.set_expected_token(<#field_type as parsable::Parsable>::get_token_name());
                                    #on_fail;
                                }
                            });
                        }

                        if attributes.ignore {
                            lines.push(quote! {
                                let #field_name = <#field_type as Default>::default();
                            });
                        } else {
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
                    }

                    let mut set_location = quote! {};

                    if located {
                        field_names.push(quote! { location });
                        named_fields.named.insert(0, create_location_field("location"));
                        set_location = quote! { let location = reader__.get_data_location(start_index__); };
                    }

                    quote! {
                        let mut field_index__ : usize = 0;
                        let mut field_failed__ = false;
                        let mut prefix_ok__ = true;
                        #(#lines)*
                        #set_location
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

                // TODO: check if variant should be skipped to avoid recursion

                let variant_name = &variant.ident;
                let mut attributes = FieldAttributes::default();
                let mut parse_prefix = quote! { true };
                let mut parse_suffix = quote! { true };
                let mut parse_method = quote! { parse(reader__) };

                if let Some((i, attr)) = variant.attrs.iter().enumerate().find(|(_, attr)| attr.path.segments.last().unwrap().ident == "parsable") {
                    let result = syn::parse2::<FieldAttributes>(attr.tokens.clone());

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
                            None => { reader__.set_expected_token(Some(format!("{:?}", #prefix))); false }
                        }
                    };
                }

                if let Some(suffix) = attributes.suffix {
                    parse_suffix = quote! {
                        match reader__.read_string(#suffix) {
                            Some(_) => { reader__.eat_spaces(); true },
                            None => { reader__.set_expected_token(Some(format!("{:?}", #suffix))); false }
                        }
                    };
                }

                if let Some(separator) = attributes.separator {
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
                                if let Some(value) = <#field_type as parsable::Parsable>::#parse_method {
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
                                    } else {
                                        reader__.set_expected_token(Some(format!("{:?}", #lit_str)));
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

                // reader__.set_expected_token(<Self as parsable::Parsable>::get_token_name());
                None
            }
        },
        _ => todo!()
    };

    let impl_token_name = match root_attributes.name {
        Some(name) => quote! {
            fn get_token_name() -> Option<String> {
                Some(#name.to_string())
            }
        },
        None => quote! { }
    };

    let result = quote! {
        #ast

        impl parsable::Parsable for #name {
            fn parse(reader__: &mut parsable::StringReader) -> Option<Self> {
                let start_index__ = reader__.get_index();

                #body
            }

            #impl_token_name
            #impl_get_location
        }
    };

    result.into()
}