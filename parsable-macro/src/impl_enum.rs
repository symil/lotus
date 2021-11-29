use proc_macro2::{Span};
use syn::*;
use quote::quote;
use crate::{field_attributes::FieldAttributes, output::Output, root_attributes::RootAttributes};

pub fn process_enum(data_enum: &mut DataEnum, root_attributes: &RootAttributes, output: &mut Output) {
    let mut lines = vec![];
    let mut impl_display_lines = vec![];
    let has_name = root_attributes.name.is_some();

    for i in 0..data_enum.variants.len() {
        let variant = &mut data_enum.variants[i];

        // TODO: check if variant should be skipped to avoid recursion

        let variant_name = &variant.ident;
        let attributes = FieldAttributes::from_field_attributes(&mut variant.attrs);
        let mut parse_prefix = quote! { true };
        let mut parse_suffix = quote! { true };
        let mut parse_method = quote! { parse_item(reader__) };
        let mut line = quote! { };

        let marker_value = match &attributes.ignore_if_marker {
            Some(name) => quote! { reader__.get_marker_value(#name) },
            None => quote! { false },
        };
        let (push_markers, pop_markers) = attributes.get_push_pop_markers();

        if let Some(prefix) = attributes.prefix {
            let prefix_consume_spaces = match attributes.consume_spaces_after_prefix {
                Some(false) => quote! { {} },
                _ => quote! { reader__.eat_spaces() },
            };

            parse_prefix = quote! {
                match reader__.read_string(#prefix) {
                    Some(_) => { #prefix_consume_spaces; true },
                    None => { reader__.set_expected_token(Some(format!("{:?}", #prefix))); false }
                }
            };
        }

        if let Some(suffix) = attributes.suffix {
            let suffix_consume_spaces = match attributes.consume_spaces_after_suffix {
                Some(false) => quote! { {} },
                _ => quote! { reader__.eat_spaces() },
            };

            parse_suffix = quote! {
                match reader__.read_string(#suffix) {
                    Some(_) => { #suffix_consume_spaces; true },
                    None => { reader__.set_expected_token(Some(format!("{:?}", #suffix))); false }
                }
            };
        }

        if let Some(separator) = attributes.separator {
            parse_method = quote! { parse_item_with_separator(reader__, #separator) };
        }

        match &variant.fields {
            Fields::Named(_) => unreachable!(),
            Fields::Unnamed(fields_unnamed) => {
                let mut value_names = vec![];

                for i in 0..fields_unnamed.unnamed.len() {
                    let value_name = Ident::new(&format!("value_{}", i), Span::call_site());

                    value_names.push(quote! { #value_name });
                }

                let mut current_block_single = quote! {
                    let suffix_ok__ = #parse_suffix;

                    if suffix_ok__ {
                        #pop_markers
                        return Some(Self::#variant_name(#(#value_names),*))
                    }
                };

                for (i, field) in fields_unnamed.unnamed.iter().enumerate().rev() {
                    let field_type = &field.ty;
                    let value_name = Ident::new(&format!("value_{}", i), Span::call_site());
                    let consume_spaces = match attributes.consume_spaces {
                        Some(false) => quote! { },
                        _ => quote! { reader__.eat_spaces(); },
                    };

                    value_names.insert(0, quote! { #value_name });

                    current_block_single = quote! {
                        if let Some(#value_name) = <#field_type as parsable::Parsable>::#parse_method {
                            #consume_spaces
                            #current_block_single
                        }
                    };
                }

                line = quote! {
                    let prefix_ok__ = #parse_prefix;

                    if prefix_ok__ {
                        #current_block_single
                    }

                    reader__.set_index(start_index__);
                };
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
                        line = quote! {
                            if let Some(_) = reader__.read_string(#lit_str) {
                                reader__.eat_spaces();
                                #pop_markers
                                return Some(Self::#variant_name);
                            } else if (! #has_name) {
                                reader__.set_expected_token(Some(format!("{:?}", #lit_str)));
                            }
                        };

                        impl_display_lines.push(quote! {
                            Self::#variant_name => #lit_str,
                        });
                    },
                    None => {
                        // emit_call_site_error!("variants with no field must have an associated string literal")
                    }
                }
            }
        }

        lines.push(quote! {
            if !(#marker_value) {
                #push_markers
                #line
                #pop_markers
            }
        });
    }

    for variant in data_enum.variants.iter_mut() {
        variant.discriminant = None;
    }

    if root_attributes.impl_display {
        output.display = Some(quote! {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = match self {
                    #(#impl_display_lines)*
                    _ => "<?>"
                };

                write!(f, "{}", string)
            }
        });
    }

    output.parse_item = quote! {
        fn parse_item(reader__: &mut parsable::StringReader) -> Option<Self> {
            let start_index__ = reader__.get_index();
            #(#lines)*

            // reader__.set_expected_token(<Self as parsable::Parsable>::get_token_name());
            None
        }
    };
}