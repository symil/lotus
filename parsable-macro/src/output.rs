use proc_macro2::{TokenStream};

#[derive(Default)]
pub struct Output {
    pub display: Option<TokenStream>,
    pub deref: Option<TokenStream>,
    pub as_str: Option<TokenStream>,
    pub parse_item: TokenStream,
    pub token_name: TokenStream,
}