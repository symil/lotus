#[macro_export]
macro_rules! create_flex_keyword_struct {
    ($struct_name:ident, $keyword:expr) => {
        #[derive(Debug)]
        pub struct $struct_name {
            pub keyword: crate::items::word::Word,
        }

        impl parsable::Parsable for $struct_name {
            fn parse_item(reader: &mut parsable::StringReader) -> Option<Self> {
                match <crate::items::word::Word as parsable::Parsable>::parse_item(reader) {
                    Some(keyword) => Some(Self { keyword }),
                    None => None,
                }
            }

            fn get_item_name() -> String {
                format!("\"{}\"", $keyword)
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = parsable::ItemLocation;

            fn deref(&self) -> &Self::Target {
                &self.keyword.location
            }
        }

        impl $struct_name {
            pub fn process(&self, context: &mut crate::program::ProgramContext) -> Option<&'static str> {
                context.completion_provider.add_keyword_completion(self, &[$keyword]);

                match self.keyword.as_str() == $keyword {
                    true => Some($keyword),
                    false => {
                        context.errors.keyword_mismatch(&self.keyword, &[$keyword]);
                        None
                    },
                }
            }
        }
    }
}