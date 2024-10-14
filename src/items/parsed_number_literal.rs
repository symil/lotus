use std::ops::Neg;
use colored::Colorize;
use parsable::parsable;
use crate::{program::{ProgramContext, Vasm, Type, BuiltinType}};

#[parsable(name="number")]
pub struct ParsedNumberLiteral {
    #[parsable(regex = r"(-|\+)?((0x[0-9abcdefABCDEF]{1,8})|((\d+(\.\d+)?)|(\.\d+))([a-z]*))")]
    pub token: String,
}

impl ParsedNumberLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut s = self.token.as_str();
        let mut is_negative = false;

        if s.starts_with("-") || s.starts_with("+") {
            is_negative = s.starts_with("-");
            s = &s[1..];
        }

        let mut value = match s.starts_with("0x") {
            true => Number::Int(i32::from_be_bytes(u32::from_str_radix(&s[2..], 16).unwrap_or(0).to_be_bytes())),
            false => {
                let (number, suffix) = split_number_suffix(s);
                let mut prefer_float = false;
                let mut prefer_display_size = false;

                if let Some(ty) = type_hint {
                    if ty.is_builtin_type(BuiltinType::Float) {
                        prefer_float = true;
                    } else if ty.is_builtin_type(BuiltinType::DisplaySize) {
                        prefer_display_size = true;
                    }
                }

                if suffix.is_empty() {
                    if prefer_float {
                        Number::Float(number.parse().unwrap())
                    } else if prefer_display_size {
                        Number::VirtualSize(number.parse().unwrap())
                    } else if number.contains(".") {
                        Number::Float(number.parse().unwrap())
                    } else {
                        match i32::from_str_radix(s, 10) {
                            Ok(n) => Number::Int(n),
                            Err(err) => {
                                context.errors.generic(self, format!("number too big"));
                                
                                Number::Int(0)
                            },
                        }
                    }
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
                            context.errors.generic(self, format!("invalid number suffix '{}'", suffix.bold()));
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
            Number::Int(value) => context.vasm().int(value).set_type(context.int_type()),
            Number::Float(value) => context.vasm().float(value).set_type(context.float_type()),
            Number::RealSize(value) => create_display_size(REAL_SIZE_VARIANT_VALUE, value, context),
            Number::VirtualSize(value) => create_display_size(VIRTUAL_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerWidthSize(value) => create_display_size(SCALED_FROM_WIDTH_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerHeightSize(value) => create_display_size(SCALED_FROM_HEIGHT_SIZE_VARIANT_VALUE, value, context),
            Number::ScaledFromContainerMinSize(value) => create_display_size(SCALED_FROM_MIN_SIZE_VARIANT_VALUE, value, context),
        };

        context.hover_provider.set_type(self, &vasm.ty);

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

    context.vasm()
        .call_static_method(&ty, "new", &[], vec![context.vasm().int(kind), context.vasm().float(value)], context)
        .set_type(&ty)
}