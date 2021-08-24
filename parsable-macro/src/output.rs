use proc_macro2::{TokenStream};

#[derive(Default)]
pub struct Output {
    pub display: Option<TokenStream>,
    pub deref: Option<TokenStream>,
    pub parse_item: TokenStream,
    pub parse_all: TokenStream,
    pub token_name: TokenStream,
}