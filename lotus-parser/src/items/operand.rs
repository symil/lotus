use parsable::{DataLocation, parsable};
use crate::program::{AccessType, ProgramContext, Wasm};
use super::{ArrayLiteral, BooleanLiteral, Expression, FloatLiteral, ObjectLiteral, StringLiteral, FullType, UnaryOperation, VarPath};

#[parsable]
pub enum Operand {
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}

impl Operand {
    pub fn get_location(&self) -> &DataLocation {
        match self {
            Operand::UnaryOperation(op) => &op.location,
            Operand::VarPath(var_path) => &var_path.location,
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.process(context),
            Operand::VarPath(var_path) => var_path.process(AccessType::Get, context),
        }
    }
}