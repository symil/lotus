use std::fmt::format;
use parsable::parsable;
use crate::program::{ProgramContext, Vasm, BuiltinType, NEW_METHOD_NAME};

const F : &'static[char] = &['F'];

#[parsable(name="color")]
pub struct ParsedColorLiteral {
    #[parsable(regex = r"#[A-Fa-f0-9]+")]
    pub token: String,
}

impl ParsedColorLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let color_type = context.get_builtin_type(BuiltinType::Color, vec![]);
        let char_count = self.token.len() - 1;

        if char_count != 3 && char_count != 6 && char_count != 8 {
            context.errors.generic(self, format!("invalid hexadecimal color"));

            return Some(context.vasm().set_type(&color_type));
        }

        let literal_var_info = context.color_literals.add(&self.token);

        Some(context.vasm()
            .get_var(&literal_var_info, None)
            .set_type(&color_type)
        )
    }
}

fn make_string(chars: &[char], c1: usize, c2: usize) -> String {
    format!("{}{}", chars[c1], chars[c2])
}

pub fn init_color_literal(color_string: &str, context: &mut ProgramContext) -> Vasm {
    let string_type = context.get_builtin_type(BuiltinType::String, vec![]);
    let chars = &color_string.chars().collect::<Vec<char>>()[1..];
    let components = match chars.len() {
        3 => [make_string(chars, 0, 0), make_string(chars, 1, 1), make_string(chars, 2, 2), make_string(F, 0, 0)],
        6 => [make_string(chars, 0, 1), make_string(chars, 2, 3), make_string(chars, 4, 5), make_string(F, 0, 0)],
        8 => [make_string(chars, 0, 1), make_string(chars, 2, 3), make_string(chars, 4, 5), make_string(chars, 6, 7)],
        _ => unreachable!()
    };

    let r = u32::from_str_radix(&components[0], 16).unwrap();
    let g = u32::from_str_radix(&components[1], 16).unwrap();
    let b = u32::from_str_radix(&components[2], 16).unwrap();
    let a = u32::from_str_radix(&components[3], 16).unwrap();
    let color_type = context.get_builtin_type(BuiltinType::Color, vec![]);


    context.vasm()
        .call_static_method(&color_type, NEW_METHOD_NAME, &[], vec![
            context.vasm().int(r),
            context.vasm().int(g),
            context.vasm().int(b),
            context.vasm().int(a)
        ], context)
        .set_type(&color_type)
}