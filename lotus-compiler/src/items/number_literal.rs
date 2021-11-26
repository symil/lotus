use std::ops::Neg;

use colored::Colorize;
use parsable::parsable;
use crate::{program::{ProgramContext, VI, Vasm}};

#[parsable(name="number")]
pub struct NumberLiteral {
    #[parsable(regex = r"(-|\+)?((0x[0-9abcdefABCDEF]{1,8})|((\d+(\.\d+)?)|(\.\d+))([a-z]*))")]
    pub value: String,
}

impl NumberLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut s = self.value.as_str();
        let mut is_negative = false;

        if s.starts_with("-") || s.starts_with("+") {
            is_negative = s.starts_with("-");
            s = &s[1..];
        }

        let mut value = match s.starts_with("0x") {
            true => Number::Int(i32::from_be_bytes(u32::from_str_radix(&s[2..], 16).unwrap().to_be_bytes())),
            false => {
                let (number, suffix) = split_number_suffix(s);

                if suffix.is_empty() && !number.contains(".") {
                    Number::Int(i32::from_str_radix(s, 10).unwrap())
                } else {
                    let value : f32 = number.parse().unwrap();

                    match suffix {
                        "r" => Number::RealSize(value),
                        "v" => Number::VirtualSize(value),
                        "w" => Number::ScaledFromContainerWidthSize(value),
                        "h" => Number::ScaledFromContainerHeightSize(value),
                        "m" => Number::ScaledFromContainerMinSize(value),
                        "f" | "" => Number::Float(value),
                        _ => {
                            context.errors.add_generic(self, format!("invalid number suffix '{}'", suffix.bold()));
                            return None;
                        }
                    }
                }
            },
        };

        if is_negative {
            value = value.neg();
        }
        
        let vasm = match value {
            Number::Int(value) => Vasm::new(context.int_type(), vec![], vec![VI::int(value)]),
            Number::Float(value) => Vasm::new(context.float_type(), vec![], vec![VI::float(value)]),
            Number::RealSize(value) => create_display_size(REAL_SIZE_VARIANT_VALUE, value, context),
            Number::VirtualSize(value) => create_display_size(VIRTUAL_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerWidthSize(value) => create_display_size(SCALED_FROM_WIDTH_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerHeightSize(value) => create_display_size(SCALED_FROM_HEIGHT_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerMinSize(value) => create_display_size(SCALED_FROM_MIN_SIZE_VARIANT_VALUE, value, context),
        };

        Some(vasm)
    }
}

const REAL_SIZE_VARIANT_VALUE : i32 = 0;
const VIRTUAL_SIZE_VARIANT_VALUE : i32 = 1;
const SCALED_FROM_WIDTH_SIZE_VARIANT_VALUE : i32 = 2;
const SCALED_FROM_HEIGHT_SIZE_VARIANT_VALUE : i32 = 3;
const SCALED_FROM_MIN_SIZE_VARIANT_VALUE : i32 = 4;

enum Number {
    Int(i32),
    Float(f32),
    RealSize(f32),
    VirtualSize(f32),
    ScaledFromContainerWidthSize(f32),
    ScaledFromContainerHeightSize(f32),
    ScaledFromContainerMinSize(f32),
}

impl Number {
    fn neg(&self) -> Number {
        match self {
            Number::Int(value) => Number::Int(value.neg()),
            Number::Float(value) => Number::Float(value.neg()),
            Number::RealSize(value) => Number::RealSize(value.neg()),
            Number::VirtualSize(value) => Number::VirtualSize(value.neg()),
            Number::ScaledFromContainerWidthSize(value) => Number::ScaledFromContainerWidthSize(value.neg()),
            Number::ScaledFromContainerHeightSize(value) => Number::ScaledFromContainerHeightSize(value.neg()),
            Number::ScaledFromContainerMinSize(value) => Number::ScaledFromContainerMinSize(value.neg()),
        }
    }
}

fn maybe_neg<T : Neg<Output = T>>(value: T, should_neg: bool) -> T {
    match should_neg {
        true => value.neg(),
        false => value,
    }
}

fn split_number_suffix(s: &str) -> (&str, &str) {
    for (i, b) in s.as_bytes().iter().enumerate() {
        let c = *b as char;

        if c >= 'a' && c <= 'z' {
            return (&s[..i], &s[i..]);
        }
    }

    (s, "")
}

fn create_display_size(kind: i32, value: f32, context: &ProgramContext) -> Vasm {
    let ty = context.display_size_type();
    let instruction = VI::call_static_method(&ty, "new", &[], vec![VI::int(kind), VI::float(value)], context);

    Vasm::new(ty, vec![], vec![instruction])
}