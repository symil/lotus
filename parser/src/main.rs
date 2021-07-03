use parser::LotusParser;

pub mod parser;
pub mod items;
pub mod located_data;
pub mod parsable;

pub use parsable::Parsable;

fn main() {
    let mut parser = LotusParser::new();

    parser.parse_root("test.lt");
}