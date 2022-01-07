use std::{fmt::format, rc::Rc};
use colored::Colorize;
use parsable::{DataLocation, parsable};
use crate::{program::{IS_METHOD_NAME, ProgramContext, Type, VariableInfo, VariableKind, Vasm}};
use super::{BinaryOperation, Identifier, ParsedType, IsOperation, AsOperation};

#[parsable(name="expression")]
pub struct Expression {
    pub operation: Box<BinaryOperation>,
    pub is_operation: Option<IsOperation>,
    pub as_operation: Option<AsOperation>,
}

#[parsable]
pub struct IsKeywordDetails {
    pub ty: ParsedType,
    #[parsable(brackets="()")]
    pub var_name: Identifier
}

impl Expression {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
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