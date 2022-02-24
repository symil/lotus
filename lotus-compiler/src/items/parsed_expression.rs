use std::{fmt::format, rc::Rc};
use colored::Colorize;
use parsable::{ItemLocation, parsable};
use crate::{program::{IS_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}};
use super::{ParsedBinaryOperation, Identifier, ParsedType, ParsedIsOperation, ParsedAsOperation};

#[parsable(name="expression")]
pub struct ParsedExpression {
    pub operation: Box<ParsedBinaryOperation>,
}

impl ParsedExpression {
    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        self.operation.collect_instancied_type_names(list, context);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(mut vasm) = self.operation.process(type_hint, context) {
            result = Some(vasm);
        }

        result
    }
}