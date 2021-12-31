use parsable::parsable;
use colored::*;
use crate::{program::{BuiltinType, CompilationError, ProgramContext, VI, Vasm}};

#[parsable(name="char")]
pub struct CharLiteral {
    #[parsable(regex = r##"'(\\.|[^'\\])*'"##)]
    pub value: String,
}

impl CharLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let chars : Vec<char> = self.value.chars().collect();
        let content = &chars[1..chars.len()-1];
        let char_opt = match content.len() {
            1 => Some(content[0]),
            2 => match content[0] {
                '\\' => escape_char(content[1], '\''),
                _ => None
            },
            _ => None
        };

        match char_opt {
            Some(c) => {
                Some(Vasm::new(context.get_builtin_type(BuiltinType::Char, vec![]), vec![], vec![VI::int(c as u32)]))
            },
            None => {
                context.errors.invalid_character(self, &content.iter().collect::<String>());
                None
            },
        }
    }
}

pub fn escape_char(c: char, quote: char) -> Option<char> {
    if c == quote {
        return Some(quote);
    }

    match c {
        '0' => Some('\0'),
        '\\' => Some('\\'),
        't' => Some('\t'),
        'n' => Some('\n'),
        _ => None
    }
}