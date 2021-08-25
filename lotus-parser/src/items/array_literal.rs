use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_BODY_FUNC_NAME, ARRAY_SET_BODY_ITEM_FUNC_NAME, ProgramContext, Type, VariableInfo, VariableKind, Wasm}, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let array_var_name = Identifier::unique("array", self).to_unique_string();
        let variables = vec![ VariableInfo::new(array_var_name.clone(), Type::Integer, VariableKind::Local) ];

        let mut all_items_ok = true;
        let mut final_type = Type::Any(0);
        let mut wat = vec![
            Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(self.items.len())]),
            Wat::tee_local(&array_var_name),
            Wat::call_from_stack(ARRAY_GET_BODY_FUNC_NAME)
        ];

        for (i, item) in self.items.iter().enumerate() {
            let mut item_ok = false;

            if let Some(item_wasm) = item.process(context) {
                if final_type.is_assignable(&item_wasm.ty, context, &mut HashMap::new()) {
                    item_ok = true;
                } else if item_wasm.ty.is_assignable(&final_type, context, &mut HashMap::new()) {
                    final_type = item_wasm.ty.clone();
                    item_ok = true;
                }

                let mut item_wat = vec![Wat::const_i32(i)];

                item_wat.extend(item_wasm.wat);

                wat.push(Wat::call(ARRAY_SET_BODY_ITEM_FUNC_NAME, item_wat));

                if !item_ok {
                    context.error(item, format!("array literal: incompatible item types `{}` and `{}`", &final_type, &item_wasm.ty));
                }
            }

            all_items_ok &= item_ok;
        }

        wat.push(wat!["drop"]);
        wat.push(Wat::get_local(&array_var_name));

        match all_items_ok {
            true => Some(Wasm::new(Type::array(final_type), wat, variables)),
            false => None
        }
    }
}