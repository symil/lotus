use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_BODY_FUNC_NAME, ProgramContext, Type, TypeOld, VariableInfo, VariableKind, Wasm}, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let array_var_name = Identifier::unique("array", self).to_unique_string();
        let array_body_var_name = Identifier::unique("array_body", self).to_unique_string();
        let variables = vec![
            VariableInfo::new(array_var_name.clone(), context.int_type(), VariableKind::Local),
            VariableInfo::new(array_body_var_name.clone(), context.int_type(), VariableKind::Local),
        ];

        let mut all_items_ok = true;
        let mut final_item_type = Type::Any;
        let mut wat = vec![
            Wat::set_local(&array_var_name, Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(self.items.len())])),
            Wat::set_local(&array_body_var_name, Wat::call(ARRAY_GET_BODY_FUNC_NAME, vec![Wat::get_local(&array_var_name)]))
        ];

        for (i, item) in self.items.iter().enumerate() {
            let mut item_ok = false;

            if let Some(item_wasm) = item.process(context) {
                if final_item_type.is_assignable_to(&item_wasm.ty, context) {
                    final_item_type = item_wasm.ty.clone();
                    item_ok = true;
                } else if item_wasm.ty.is_assignable_to(&final_item_type, context) {
                    item_ok = true;
                }

                let func_name = final_item_type.pointer_set_function_name();

                wat.extend(item_wasm.wat);
                wat.push(Wat::get_local(&array_body_var_name));
                wat.push(Wat::const_i32(i));
                wat.push(Wat::call_from_stack(func_name));

                if !item_ok {
                    context.errors.add(item, format!("incompatible item types `{}` and `{}`", &final_item_type, &item_wasm.ty));
                }
            }

            all_items_ok &= item_ok;
        }

        wat.push(Wat::get_local(&array_var_name));

        match all_items_ok {
            true => Some(Wasm::new(context.array_type(final_item_type), wat, variables)),
            false => None
        }
    }
}