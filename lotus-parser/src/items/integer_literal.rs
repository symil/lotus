use parsable::parsable;
use crate::{program::{ProgramContext, VI, Vasm}};

#[parsable(name="integer")]
pub struct IntegerLiteral {
    #[parsable(regex = r"(-|\+)?((0x[0-9abcdefABCDEF]{1,8})|(\d+))")]
    pub value: String,
}

impl IntegerLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut s = self.value.as_str();
        let mut is_negative = false;

        if s.starts_with("-") || s.starts_with("+") {
            is_negative = s.starts_with("-");
            s = &s[1..];
        }
        
        let mut i32_value = match self.value.contains("0x") {
            true => i32::from_be_bytes(u32::from_str_radix(&s[2..], 16).unwrap().to_be_bytes()),
            false => i32::from_str_radix(s, 10).unwrap(),
        };
        
        if is_negative {
            i32_value *= -1;
        }

        Some(Vasm::new(context.int_type(), vec![], vec![VI::int(i32_value)]))
    }
}