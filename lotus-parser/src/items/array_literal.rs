use std::collections::HashMap;
use parsable::parsable;
use crate::{items::Identifier, program::{BuiltinType, GET_BODY_FUNC_NAME, CREATE_METHOD_NAME, ProgramContext, SET_AT_INDEX_FUNC_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, vasm, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        for item in &self.items {
            item.collected_instancied_type_names(list);
        }
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let array_var = VariableInfo::new(Identifier::unique("array", self), context.int_type(), VariableKind::Local);
        let array_body_var = VariableInfo::new(Identifier::unique("array_body", self), context.int_type(), VariableKind::Local);
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
                } else if item_vasm.ty.is_assignable_to(&final_item_type) {
                    item_ok = true;
                } else if type_hint.is_none() && final_item_type.is_assignable_to(&item_vasm.ty) {
                    final_item_type = item_vasm.ty.clone();
                    item_ok = true;
                }

                if !item_ok {
                    context.errors.add(item, format!("expected `{}`, got `{}`", &final_item_type, &item_vasm.ty));
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
            VI::set(&array_var, VI::call_static_method(&final_array_type, CREATE_METHOD_NAME, &[], vec![VI::int(self.items.len())], context)),
            VI::set(&array_body_var, VI::call_regular_method(&final_array_type, GET_BODY_FUNC_NAME, &[], vec![VI::get(&array_var)], context)),
        ];

        for (i, item_vasm) in item_vasm_list.into_iter().enumerate() {
            instructions.extend(vec![
                VI::get(&array_body_var),
                VI::call_regular_method(&final_pointer_type, SET_AT_INDEX_FUNC_NAME, &[], vasm![VI::int(i), item_vasm], context)
            ]);
        }

        instructions.push(VI::get(&array_var));

        Some(Vasm::new(final_array_type, variables, instructions))
    }
}