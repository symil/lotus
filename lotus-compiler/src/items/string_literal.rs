use parsable::{DataLocation, parsable};
use crate::{items::escape_char, program::{BuiltinType, CREATE_METHOD_NAME, ProgramContext, SET_CHAR_FUNC_NAME, VI, Vasm, CompilationError}, wat};

#[parsable(name="string")]
pub struct StringLiteral {
    #[parsable(regex = r##""(\\.|[^"\\])*""##)]
    pub value: String
}

impl StringLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match make_string_value_from_literal(&self.location, &self.value[1..self.value.len()-1], context) {
            Some(vasm) => Some(vasm),
            None => Some(Vasm::new(context.get_builtin_type(BuiltinType::String, vec![]), vec![], vec![])),
        }
    }
}

pub fn make_string_value_from_literal(location: &DataLocation, literal: &str, context: &mut ProgramContext) -> Option<Vasm> {
    match get_string_vasm_from_literal(Some(location), literal, context) {
        Ok(vasm) => Some(vasm),
        Err(errors) => {
            for error in errors {
                context.errors.add(error);
            }

            None
        },
    }
}

pub fn make_string_value_from_literal_unchecked(literal: &str, context: &ProgramContext) -> Vasm {
    get_string_vasm_from_literal(None, literal, context).unwrap()
}

fn get_string_vasm_from_literal(location: Option<&DataLocation>, literal: &str, context: &ProgramContext) -> Result<Vasm, Vec<CompilationError>> {
    let mut chars : Vec<char> = literal.chars().collect();
    let mut escaping = false;
    let mut unescaped_chars = vec![];
    let mut errors = vec![];

    for c in chars {
        if escaping {
            if let Some(escaped_char) = escape_char(c, '"') {
                unescaped_chars.push(escaped_char as u32);
            } else if let Some(location) = &location {
                errors.push(CompilationError::generic(location, format!("invalid character '\\{}'", c)));
            }

            escaping = false;
        } else if c == '\\' {
            escaping = true;
        } else {
            unescaped_chars.push(c as u32);
        }
    }

    let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
    let mut content = vec![
        VI::call_static_method(&string_type, CREATE_METHOD_NAME, &[], vec![VI::int(unescaped_chars.len())], context)
    ];

    for (i, code) in unescaped_chars.into_iter().enumerate() {
        content.push(
            VI::call_regular_method(&string_type, SET_CHAR_FUNC_NAME, &[], vec![VI::int(i), VI::int(code)], context),
        );
    }

    match errors.is_empty() {
        true => Ok(Vasm::new(string_type, vec![], content)),
        false => Err(errors),
    }
}