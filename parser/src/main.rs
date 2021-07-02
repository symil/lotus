use parser::LotusParser;

pub mod grammar;
pub mod parser;
pub mod items;
pub mod located_data;
pub mod parsable;

fn main() {
    let mut parser = LotusParser::new();

    parser.parse_root("test.lt");
}