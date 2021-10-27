use parsable::{DataLocation, parsable};
use crate::program::{AccessType, ProgramContext, Type, Vasm};
use super::{ArrayLiteral, BooleanLiteral, Expression, FloatLiteral, FullType, Identifier, ObjectLiteral, StringLiteral, UnaryOperation, VarPath};

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

    pub fn has_side_effects(&self) -> bool {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.has_side_effects(),
            Operand::VarPath(var_path) => var_path.has_side_effects(),
        }
    }

    pub fn as_single_local_variable(&self) -> Option<&Identifier> {
        match self {
            Operand::UnaryOperation(_) => None,
            Operand::VarPath(var_path) => var_path.as_single_local_variable(),
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.collected_instancied_type_names(list),
            Operand::VarPath(var_path) => var_path.collected_instancied_type_names(list),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.process(type_hint, context),
            Operand::VarPath(var_path) => var_path.process(type_hint, AccessType::Get, context),
        }
    }
}