use parsable::{DataLocation, parsable};
use crate::program::{AccessType, ProgramContext, Type, Vasm};
use super::{Action, ArrayLiteral, Assignment, BooleanLiteral, Expression, ForBlock, Identifier, IfBlock, IterAncestors, IterFields, IterVariants, ObjectLiteral, ParsedType, StringLiteral, UnaryOperation, VarDeclaration, VarPath, WhileBlock};

#[parsable]
pub enum Operand {
    UnaryOperation(UnaryOperation),
    Assignment(Assignment)
}

impl Operand {
    pub fn get_location(&self) -> &DataLocation {
        match self {
            Operand::UnaryOperation(op) => &op.location,
            Operand::Assignment(assignment) => &assignment.location,
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.collected_instancied_type_names(list, context),
            Operand::Assignment(assignment) => assignment.collected_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            Operand::UnaryOperation(unary_operation) => unary_operation.process(type_hint, context),
            Operand::Assignment(assignment) => assignment.process(type_hint, context),
        }
    }
}