use crate::{ParseError, string_reader::{ParseOptions, StringReader}, end_of_file::EndOfFile, ItemLocation};

pub trait Parsable : Sized {
    fn parse_item(reader: &mut StringReader) -> Option<Self>;

    #[allow(unused_variables)]
    fn parse_item_without_consuming_spaces(reader: &mut StringReader) -> Option<Self> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn parse_item_with_separator(reader: &mut StringReader, separator: &'static str) -> Option<Self> {
        unimplemented!()
    }

    fn location(&self) -> &ItemLocation {
        panic!("type {} has no location", std::any::type_name::<Self>());
    }

    fn get_item_name() -> String;

    // fn get_wrapped_name() -> String {
    //     let name = Self::item_name();
    //     let wrapper = Self::item_name_wrapper();

    //     match wrapper.as_bytes().get(0) {
    //         Some(first) => {
    //             let second = wrapper.as_bytes().get(1).unwrap_or(first);
    //             let result = format!("{}{}{}", *first as char, name, *second as char);

    //             result
    //         },
    //         None => name.to_string(),
    //     }
    // }

    fn parse(string: String, options: ParseOptions) -> Result<Self, ParseError> {
        let mut reader = StringReader::new(string, options);

        reader.eat_spaces();

        match Self::parse_item(&mut reader) {
            Some(value) => match reader.is_finished() {
                true => Ok(value),
                false => {
                    reader.set_expected_item::<EndOfFile>();
                    Err(reader.get_error())
                }
            },
            None => {
                reader.set_expected_item::<Self>();
                Err(reader.get_error())
            }
        }
    }
}