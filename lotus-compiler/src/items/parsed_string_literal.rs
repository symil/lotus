use parsable::{DataLocation, parsable};
use crate::{items::escape_char, program::{BuiltinType, ProgramContext, SET_CHAR_FUNC_NAME, Vasm, CompilationError, STRING_CREATE_METHOD_NAME}, wat, utils::FlexRef};

#[parsable(name="string")]
pub struct ParsedStringLiteral {
    #[parsable(regex = r##""(\\.|[^"\\])*""##)]
    pub token: String
}

impl ParsedStringLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        match make_string_value_from_literal(&self.location, &self.token[1..self.token.len()-1], context) {
            Some(vasm) => Some(vasm),
            None => Some(context.vasm()
                .set_type(context.get_builtin_type(BuiltinType::String, vec![]))
            )
        }
    }
}

pub fn make_string_value_from_literal(location: &DataLocation, literal: &str, context: &mut ProgramContext) -> Option<Vasm> {
    Some(get_string_vasm_from_literal(Some(location), literal, FlexRef::Mut(context)))
}

pub fn make_string_value_from_literal_unchecked(literal: &str, context: &ProgramContext) -> Vasm {
    get_string_vasm_from_literal(None, literal, FlexRef::Const(context))
}

fn get_string_vasm_from_literal(location: Option<&DataLocation>, literal: &str, mut context_ref: FlexRef<ProgramContext>) -> Vasm {
    let mut chars : Vec<char> = literal.chars().collect();
    let mut escaping = false;
    let mut unescaped_chars = vec![];

    for c in chars {
        if escaping {
            if let Some(escaped_char) = escape_char(c, '"') {
                unescaped_chars.push(escaped_char as u32);
            } else if let Some(location) = &location {
                if let FlexRef::Mut(context) = &mut context_ref {
                    context.errors.invalid_character(location, &format!("\\{}", c));
                }
            }

            escaping = false;
        } else if c == '\\' {
            escaping = true;
        } else {
            unescaped_chars.push(c as u32);
        }
    }

    let context = context_ref.as_ref();
    let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
    let mut result = context.vasm()
        .call_static_method(&string_type, STRING_CREATE_METHOD_NAME, &[], vec![context.vasm().int(unescaped_chars.len())], context)
        .set_type(&string_type);

    for (i, code) in unescaped_chars.into_iter().enumerate() {
        result = result
            .call_regular_method(&string_type, SET_CHAR_FUNC_NAME, &[], vec![context.vasm().int(i), context.vasm().int(code)], context);
    }

    result
}