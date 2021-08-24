use proc_macro2::TokenStream;
use proc_macro_error::emit_call_site_error;
use quote::quote;
use syn::{*, parse::{Parse, ParseStream}};

#[derive(Default)]
pub struct FieldAttributes {
    pub regex: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub min: Option<usize>,
    pub separator: Option<String>,
    pub optional: Option<bool>,
    pub set_marker: Option<LitStr>,
    pub unset_marker: Option<LitStr>,
    pub ignore_if_marker: Option<LitStr>,
    pub ignore: bool
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
                    "set_marker" => attributes.set_marker = Some(content.parse::<LitStr>()?),
                    "unset_marker" => attributes.unset_marker = Some(content.parse::<LitStr>()?),
                    "ignore_if_marker" => attributes.ignore_if_marker = Some(content.parse::<LitStr>()?),
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

impl FieldAttributes {
    pub fn from_field_attributes(attrs: &mut Vec<Attribute>) -> Self {
        let mut attributes = Self::default();

        if let Some((i, attr)) = attrs.iter().enumerate().find(|(_, attr)| attr.path.segments.last().unwrap().ident == "parsable") {
            let result = syn::parse2::<FieldAttributes>(attr.tokens.clone());

            match result {
                Ok(value) => attributes = value,
                Err(error) => emit_call_site_error!(error)
            };

            attrs.remove(i);
        }

        attributes
    }

    pub fn get_push_pop_markers(&self) -> (TokenStream, TokenStream) {
        let mut push_markers = vec![];
        let mut pop_markers = vec![];

        if let Some(name) = &self.set_marker {
            push_markers.push(quote! { reader__.push_marker_value(#name, true); });
            pop_markers.push(quote! { reader__.pop_marker_value(#name); });
        }

        if let Some(name) = &self.unset_marker {
            push_markers.push(quote! { reader__.push_marker_value(#name, false); });
            pop_markers.push(quote! { reader__.pop_marker_value(#name); });
        }

        (
            quote! { #(#push_markers)* },
            quote! { #(#pop_markers)* },
        )
    }
}