use parser::LotusParser;

mod parser;
mod items;

fn main() {
    let mut parser = LotusParser::new();

    parser.parse_root("test.lt");
}