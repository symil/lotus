use syn::{*, parse::{Parse, ParseStream}};

// TODO: add prefix and suffix
pub struct RootAttributes {
    pub located: bool,
    pub impl_display: bool,
    pub none_cascade: bool,
    pub name: Option<String>
}

impl Default for RootAttributes {
    fn default() -> Self {
        Self {
            located: true,
            impl_display: false,
            none_cascade: false,
            name: None
        }
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
                "impl_display" => attributes.impl_display = content.parse::<LitBool>()?.value(),
                "none_cascade" => attributes.none_cascade = content.parse::<LitBool>()?.value(),
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