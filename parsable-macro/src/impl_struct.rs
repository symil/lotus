use syn::{*, parse::{Parse, ParseStream}};
use quote::quote;
use crate::{field_attributes::FieldAttributes, output::Output, root_attributes::RootAttributes, utils::is_type};

struct Wrapper {
    field: Field
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let field = Field::parse_named(input)?;

        Ok(Self { field })
    }
}

pub fn create_location_field(field_name: &str) -> Field {
    let string = format!("pub {}: parsable::DataLocation", field_name);
    let result : Result<Wrapper> = syn::parse_str(&string);

    result.unwrap().field
}

pub fn process_struct(data_struct: &mut DataStruct, attributes: &RootAttributes, output: &mut Output) {
    output.deref = Some(quote! {
        fn deref(&self) -> &Self::Target {
            &self.location
        }
    });

    match &mut data_struct.fields {
        Fields::Named(named_fields) => {
            let mut field_names = vec![];
            let mut lines = vec![];

            for field in named_fields.named.iter_mut() {
                let attributes = FieldAttributes::from_field_attributes(&mut field.attrs);
                let (push_markers, pop_markers) = attributes.get_push_pop_markers();
                let is_vec = is_type(&field.ty, "Vec");
                let is_option = is_type(&field.ty, "Option");

                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                field_names.push(quote! { #field_name });

                let optional = is_option || attributes.optional.map_or(false, |value| value);
                let on_success = quote! { reader__.eat_spaces(); };
                let mut handle_failure = quote! {};
                let mut on_fail = quote ! {
                    reader__.set_index(start_index__);
                    #pop_markers
                    return None;
                };

                if optional {
                    on_fail = quote! {
                        field_failed__ = true;
                        reader__.set_index(field_index__);
                        <#field_type as Default>::default()
                    };

                    if attributes.suffix.is_some() {
                        handle_failure = quote! {
                            if field_failed__ {
                                #field_name = <#field_type as Default>::default();
                            }
                        }
                    }
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
                                field_failed__ = true;
                                #on_fail;
                            }
                        };
                    },
                    None => quote! {}
                };
                let suffix_parsing = match attributes.suffix {
                    Some(suffix) => quote! {
                        if !field_failed__ {
                            match reader__.read_string(#suffix) {
                                Some(_) => reader__.eat_spaces(),
                                None => {
                                    reader__.set_expected_token(Some(format!("{:?}", #suffix)));
                                    #on_fail;
                                }
                            };
                        }
                    },
                    None => quote! {}
                };
                let mut followed_by_parsing = quote! {};

                if let Some(followed_by) = &attributes.followed_by {
                    followed_by_parsing = quote! {
                        if !field_failed__ && !reader__.peek_regex(#followed_by) {
                            reader__.set_expected_token(Some(format!("{:?}", #followed_by)));
                            #on_fail;
                        }
                    };
                }

                let mut parse_method = quote! { parse_item(reader__) };

                if let Some(separator) = attributes.separator {
                    parse_method = quote! { parse_item_with_separator(reader__, #separator) };
                }

                let mut assignment = quote! {
                    let mut #field_name = match <#field_type as parsable::Parsable>::#parse_method {
                        Some(value) => value,
                        None => {
                            reader__.set_expected_token(<#field_type as parsable::Parsable>::get_token_name());
                            #on_fail
                        }
                    };
                };

                if has_prefix && optional {
                    assignment = quote! {
                        let mut #field_name = match prefix_ok__ {
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

                let make_field_from_string = match is_option {
                    true => quote! { Some(value) },
                    false => quote! { value },
                };

                if let Some(pattern) = attributes.regex {
                    assignment = quote! {
                        let #field_name = match reader__.read_regex(#pattern) {
                            Some(value) => match <String as std::str::FromStr>::from_str(value) {
                                Ok(value) => #make_field_from_string,
                                Err(_) => { #on_fail }
                            },
                            None => { #on_fail }
                        };
                    }
                } else if let Some(literal) = attributes.value {
                    assignment = quote! {
                        let #field_name = match reader__.read_string(#literal) {
                            Some(value) => match <String as std::str::FromStr>::from_str(value) {
                                Ok(value) => #make_field_from_string,
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
                        #push_markers
                        field_failed__ = false;
                        field_index__ = reader__.get_index();
                        prefix_ok__ = true;
                        #prefix_parsing
                        #assignment
                        #(#check)*
                        #suffix_parsing
                        #followed_by_parsing
                        #on_success
                        #handle_failure
                        #pop_markers
                    });
                }
            }

            let mut set_location = quote! {};

            if attributes.located {
                field_names.push(quote! { location });
                named_fields.named.insert(0, create_location_field("location"));
                set_location = quote! { let location = reader__.get_data_location(start_index__); };
            }

            output.parse_item = quote! {
                fn parse_item(reader__: &mut parsable::StringReader) -> Option<Self> {
                    let start_index__ = reader__.get_index();
                    let mut field_index__ : usize = 0;
                    let mut field_failed__ = false;
                    let mut prefix_ok__ = true;
                    #(#lines)*
                    #set_location
                    Some(Self { #(#field_names),* })
                }
            };

        },
        Fields::Unnamed(_) => unreachable!(),
        Fields::Unit => unreachable!()
    }
}