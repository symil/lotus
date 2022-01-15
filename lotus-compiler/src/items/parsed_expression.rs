use std::{fmt::format, rc::Rc};
use colored::Colorize;
use parsable::{ItemLocation, parsable};
use crate::{program::{IS_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}};
use super::{ParsedBinaryOperation, Identifier, ParsedType, ParsedIsOperation, ParsedAsOperation};

#[parsable(name="expression")]
pub struct ParsedExpression {
    pub operation: Box<ParsedBinaryOperation>,
    pub is_operation: Option<ParsedIsOperation>,
    pub as_operation: Option<ParsedAsOperation>,
}

impl ParsedExpression {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        self.operation.collected_instancied_type_names(list, context);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(mut vasm) = self.operation.process(type_hint, context) {
            if let Some(is_operation) = &self.is_operation {
                if let Some(is_vasm) = is_operation.process(&vasm.ty, context) {
                    vasm = vasm.append(is_vasm);
                }
            }

            if let Some(as_operation) = &self.as_operation {
                if let Some(as_vasm) = as_operation.process(&vasm.ty, context) {
                    vasm = vasm.append(as_vasm);
                }
            }

            result = Some(vasm);
        }

        result
    }
}