use std::{fmt::format, rc::Rc};
use colored::Colorize;
use parsable::{DataLocation, parsable};
use crate::{program::{IS_METHOD_NAME, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}};
use super::{BinaryOperation, Identifier, ParsedType};

#[parsable(name="expression")]
pub struct Expression {
    pub operation: Box<BinaryOperation>,
    #[parsable(prefix="is")]
    pub is_details: Option<IsKeywordDetails>,
    #[parsable(prefix="as")]
    pub as_type: Option<ParsedType>
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
            if let Some(is_details) = &self.is_details {
                if let Some(target_type) = is_details.ty.process(true, context) {
                    match target_type.is_object() {
                        true => {
                            let var_info = context.declare_local_variable(is_details.var_name.clone(), target_type.clone());

                            vasm.ty = context.bool_type();
                            vasm.variables.push(var_info.clone());
                            vasm.instructions.extend(vec![
                                VI::tee_tmp_var(&var_info),
                                VI::call_static_method(&target_type, IS_METHOD_NAME, &[], vec![], context)
                            ]);
                        },
                        false => {
                            context.errors.expected_class_type(&is_details.ty, &target_type);
                        }
                    }
                }
            }

            if let Some(as_type) = &self.as_type {
                if let Some(target_type) = as_type.process(true, context) {
                    vasm.ty = target_type;
                }
            }

            result = Some(vasm);
        }

        result
    }
}