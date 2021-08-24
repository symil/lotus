use proc_macro_error::emit_call_site_error;
use proc_macro2::{Span};
use syn::*;
use quote::quote;
use crate::{field_attributes::FieldAttributes, output::Output, root_attributes::RootAttributes};

pub fn process_enum(data_enum: &mut DataEnum, attributes: &RootAttributes, output: &mut Output) {
    let mut lines = vec![];
    let mut impl_display_lines = vec![];

    for i in 0..data_enum.variants.len() {
        let variant = &mut data_enum.variants[i];

        // TODO: check if variant should be skipped to avoid recursion

        let variant_name = &variant.ident;
        let mut attributes = FieldAttributes::default();
        let mut parse_prefix = quote! { true };
        let mut parse_suffix = quote! { true };
        let mut parse_method = quote! { parse_item(reader__) };

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
                let mut value_names = vec![];

                for i in 0..fields_unnamed.unnamed.len() {
                    let value_name = Ident::new(&format!("value_{}", i), Span::call_site());

                    value_names.push(quote! { #value_name });
                }

                let mut current_block = quote! {
                    let suffix_ok__ = #parse_suffix;

                    if suffix_ok__ {
                        return Some(Self::#variant_name(#(#value_names),*))
                    }
                };

                for (i, field) in fields_unnamed.unnamed.iter().enumerate().rev() {
                    let field_type = &field.ty;
                    let value_name = Ident::new(&format!("value_{}", i), Span::call_site());

                    value_names.insert(0, quote! { #value_name });

                    current_block = quote! {
                        if let Some(#value_name) = <#field_type as parsable::Parsable>::#parse_method {
                            reader__.eat_spaces();

                            #current_block
                        }
                    };
                }

                lines.push(quote! {
                    let prefix_ok__ = #parse_prefix;

                    if prefix_ok__ {
                        #current_block
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

                        impl_display_lines.push(quote! {
                            Self::#variant_name => #lit_str,
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

    if attributes.impl_display {
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