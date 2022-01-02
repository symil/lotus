use std::collections::HashMap;
use parsable::parsable;
use crate::{items::Identifier, program::{BuiltinType, CompilationError, GET_BODY_FUNC_NAME, ProgramContext, SET_AT_INDEX_FUNC_NAME, Type, VI, VariableInfo, VariableKind, Vasm, ARRAY_CREATE_METHOD_NAME, PUSH_UNCHECKED_METHOD_NAME}, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        for item in &self.items {
            item.collected_instancied_type_names(list, context);
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let array_var = VariableInfo::tmp("array", context.int_type());
        let variables = vec![ array_var.clone() ];

        let mut all_items_ok = true;
        let mut final_item_type = match type_hint {
            Some(ty) => match ty.get_array_item() {
                Some(item_type) => item_type.clone(),
                None => Type::Any
            },
            None => Type::Any,
        };
        let mut item_vasm_list = vec![];

        for item in self.items.iter() {
            let mut item_ok = false;
            let item_type_hint = match &final_item_type {
                Type::Any => None,
                _ => Some(&final_item_type)
            };

            if let Some(item_vasm) = item.process(item_type_hint, context) {
                if final_item_type == Type::Any {
                    final_item_type = item_vasm.ty.clone();
                    item_ok = true;
                } else if let Some(common_type) = item_vasm.ty.get_common_type(&final_item_type) {
                    final_item_type = common_type;
                    item_ok = true;
                }

                if !item_ok {
                    context.errors.type_mismatch(item, &final_item_type, &item_vasm.ty);
                    all_items_ok = false;
                } else {
                    item_vasm_list.push(item_vasm);
                }
            }
        }

        if !all_items_ok {
            return None;
        }

        let final_array_type = context.get_builtin_type(BuiltinType::Array, vec![final_item_type.clone()]);
        let mut result = context.vasm()
            .declare_variable(&array_var)
            .call_static_method(&final_array_type, ARRAY_CREATE_METHOD_NAME, &[], vec![context.vasm().int(self.items.len())], context)
            .set_tmp_var(&array_var)
            .set_type(&final_array_type);

        for item_vasm in item_vasm_list {
            result = result
                .get_tmp_var(&array_var)
                .call_regular_method(&final_array_type, PUSH_UNCHECKED_METHOD_NAME, &[], vec![item_vasm], context);
        }

        result = result
            .get_tmp_var(&array_var);

        Some(result)
    }
}