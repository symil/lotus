use std::{fmt::format, rc::Rc};

use colored::Colorize;
use parsable::{DataLocation, parsable};
use crate::{items::ValueOrType, program::{IS_METHOD_NAME, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}};
use super::{BinaryOperation, ParsedType, Identifier, Macro};

#[parsable]
pub struct Expression {
    pub operation: BinaryOperation,
    #[parsable(prefix="is")]
    pub is_type: Option<TargetType>,
    #[parsable(prefix="as")]
    pub as_type: Option<TargetType>
}

#[parsable]
pub enum TargetType {
    Type(ParsedType),
    Macro(Macro)
}

impl Expression {
    pub fn has_side_effects(&self) -> bool {
        self.operation.has_side_effects()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.operation.collected_instancied_type_names(list);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(mut vasm) = self.operation.process(type_hint, context) {
            if let Some(is_type) = &self.is_type {
                if let Some((target_type, location)) = is_type.process(context) {
                    match target_type.is_object() {
                        true => {
                            vasm.ty = context.bool_type();
                            vasm.instructions.push(VI::call_static_method(&target_type, IS_METHOD_NAME, &[], vec![], context));

                            if let Some(var_name) = self.operation.as_single_local_variable() {
                                if let Some(var_info) = context.get_var_info(var_name) {
                                    if var_info.kind != VariableKind::Global {
                                        context.push_var(&Rc::new(VariableInfo {
                                            name: var_info.name.clone(),
                                            ty: target_type,
                                            kind: var_info.kind,
                                            wasm_name: var_info.wasm_name.clone(),
                                        }));
                                    }
                                }
                            }
                        },
                        false => {
                            context.errors.add(location, format!("expected class type, got `{}`", &target_type));
                        }
                    }
                }
            }

            if let Some(as_type) = &self.as_type {
                if let Some((target_type, location)) = as_type.process(context) {
                    vasm.ty = target_type;
                }
            }

            result = Some(vasm);
        }

        result
    }
}

impl TargetType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<(Type, &DataLocation)> {
        match self {
            TargetType::Type(full_type) => match full_type.process(true, context) {
                Some(ty) => Some((ty, full_type)),
                None => None,
            },
            TargetType::Macro(mac) => match mac.process_as_value(context) {
                Some(value_or_type) => match value_or_type {
                    ValueOrType::Value(_) => {
                        context.errors.add(mac, format!("expected type, got expression"));
                        None
                    },
                    ValueOrType::Type(ty) => Some((ty, mac)),
                },
                None => None,
            },
        }
    }
}