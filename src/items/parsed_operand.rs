use parsable::{ItemLocation, parsable};
use crate::program::{AccessType, ProgramContext, Type, Vasm};
use super::{ParsedAction, ParsedArrayLiteral, ParsedOperandBody, ParsedBooleanLiteral, ParsedExpression, ParsedForBlock, Identifier, ParsedIfBlock, ParsedIterAncestorsBlock, ParsedIterFieldsBlock, ParsedIterVariantsBlock, ParsedObjectLiteral, ParsedType, ParsedStringLiteral, ParsedUnaryOperation, ParsedVarDeclaration, ParsedVarPath, ParsedWhileBlock};

#[parsable]
pub enum ParsedOperand {
    UnaryOperation(ParsedUnaryOperation),
    OperandBody(ParsedOperandBody)
}

impl ParsedOperand {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        match self {
            ParsedOperand::UnaryOperation(unary_operation) => unary_operation.collect_instancied_type_names(list, context),
            ParsedOperand::OperandBody(assignment) => assignment.collect_instancied_type_names(list, context),
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        match self {
            ParsedOperand::UnaryOperation(unary_operation) => unary_operation.process(type_hint, context),
            ParsedOperand::OperandBody(assignment) => assignment.process(type_hint, context),
        }
    }
}