use parsable::parsable;
use crate::{generation::{ARRAY_LENGTH_FUNC_NAME, Wat, ToWatVec, ToWat}, merge, program::{ProgramContext, Type, Wasm}, wat};
use super::{Operand, UnaryOperator};

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

impl UnaryOperation {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;

        if let Some(operand_wasm) = self.operand.process(context) {
            let wat = match &self.operator {
                UnaryOperator::Not => match &operand_wasm.ty {
                    Type::Pointer => Some(wat!["i32.eqz"]),
                    Type::Boolean => Some(wat!["i32.eqz"]),
                    Type::Integer => Some(wat!["i32.eq", Wat::const_i32(i32::MIN)]),
                    Type::Float => Some(wat!["i32.eq", wat!["i32.reinterpret_f32"], wat!["i32.reinterpret_f32", wat!["f32.const", "nan"]]]),
                    Type::String => Some(wat!["i32.eqz", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![])]),
                    Type::Struct(_) => Some(wat!["i32.eqz"]),
                    Type::Function(_, _) => Some(wat!["i32.eqz", Wat::call(ARRAY_LENGTH_FUNC_NAME, vec![])]),
                    _ => None
                },
                UnaryOperator::Plus => match &operand_wasm.ty {
                    Type::Integer => Some(wat!["nop"]),
                    Type::Float => Some(wat!["nop"]),
                    _ => None
                },
                UnaryOperator::Minus => match &operand_wasm.ty {
                    Type::Integer => Some(wat!["i32.mul", Wat::const_i32(-1)]),
                    Type::Float => Some(wat!["f32.neg"]),
                    _ => None
                },
            };

            if let Some(operand_wat) = wat {
                result = Some(Wasm::typed(operand_wasm.ty, merge![operand_wasm.wat, operand_wat]));
            } else {
                context.error(&self, format!("cannot apply unary operator `{}` on type `{}`", &self.operator, &operand_wasm.ty));
            }
        }

        result
    }
}