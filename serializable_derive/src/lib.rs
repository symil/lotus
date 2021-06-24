use proc_macro::{TokenStream};
use quote::quote;
use syn::{self, Data, Fields};

#[proc_macro_derive(Serializable)]
pub fn serializable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_serializable_macro(&ast)
}

fn impl_serializable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut serialize_statements = vec![];

    match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(named_fields) => {
                    for field in &named_fields.named {
                        let name = &field.ident.as_ref().unwrap();
                        let ty = &field.ty;

                        serialize_statements.push(quote! {
                            <#ty>::write_bytes(&value.#name, bytes);
                        });
                    }
                },
                Fields::Unnamed(fields) => {
                    
                },
                Fields::Unit => {}
            }
        },
        Data::Enum(data_enum) => {

        },
        Data::Union(data_union) => todo!()
    };

    let gen = quote! {
        impl Serializable for #name {
            fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
                #(#serialize_statements)*
            }

            fn read_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
                None
            }
        }
    };

    gen.into()
}