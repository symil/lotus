use std::collections::HashMap;
use parsable::parsable;
use crate::{items::Identifier, program::{BuiltinType, CREATE_METHOD_NAME, CompilationError, GET_BODY_FUNC_NAME, ProgramContext, SET_AT_INDEX_FUNC_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, vasm, wat};
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
        let array_var = VariableInfo::tmp("array", Type::Int);
        let array_body_var = VariableInfo::tmp("array_body", Type::Int);
        let variables = vec![
            array_var.clone(),
            array_body_var.clone()
        ];

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
        let final_pointer_type = context.get_builtin_type(BuiltinType::Pointer, vec![final_item_type.clone()]);
        let mut instructions = vec![
            VI::call_static_method(&final_array_type, CREATE_METHOD_NAME, &[], vec![VI::int(self.items.len())], context),
            VI::set_tmp_var(&array_var),
            VI::call_regular_method(&final_array_type, GET_BODY_FUNC_NAME, &[], vec![VI::get_tmp_var(&array_var)], context),
            VI::set_tmp_var(&array_body_var),
        ];

        for (i, item_vasm) in item_vasm_list.into_iter().enumerate() {
            instructions.extend(vec![
                VI::get_tmp_var(&array_body_var),
                VI::call_regular_method(&final_pointer_type, SET_AT_INDEX_FUNC_NAME, &[], vasm![VI::int(i), item_vasm], context)
            ]);
        }

        instructions.push(VI::get_tmp_var(&array_var));

        Some(Vasm::new(final_array_type, variables, instructions))
    }
}