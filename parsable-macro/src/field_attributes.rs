use syn::{*, parse::{Parse, ParseStream}};

#[derive(Default)]
pub struct FieldAttributes {
    pub regex: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub min: Option<usize>,
    pub separator: Option<String>,
    pub optional: Option<bool>,
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