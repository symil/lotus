use parsable::{DataLocation, parsable};
use crate::program::{AccessType, ProgramContext, Type, Vasm};
use super::{ParsedAction, ParsedArrayLiteral, ParsedAssignment, ParsedBooleanLiteral, ParsedExpression, ParsedForBlock, Identifier, ParsedIfBlock, ParsedIterAncestorsBlock, ParsedIterFieldsBlock, ParsedIterVariantsBlock, ParsedObjectLiteral, ParsedType, ParsedStringLiteral, ParsedUnaryOperation, ParsedVarDeclaration, ParsedVarPath, ParsedWhileBlock};

#[parsable]
pub enum ParsedOperand {
    UnaryOperation(ParsedUnaryOperation),
    Assignment(ParsedAssignment)
}

impl ParsedOperand {
    pub fn get_location(&self) -> &DataLocation {
        match self {
            ParsedOperand::UnaryOperation(op) => &op.location,
            ParsedOperand::Assignment(assignment) => &assignment.location,
        }
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self {
            ParsedOperand::UnaryOperation(unary_operation) => unary_operation.collected_instancied_type_names(list, context),
            ParsedOperand::Assignment(assignment) => assignment.collected_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedOperand::UnaryOperation(unary_operation) => unary_operation.process(type_hint, context),
            ParsedOperand::Assignment(assignment) => assignment.process(type_hint, context),
        }
    }
}