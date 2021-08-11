use std::collections::HashMap;

use parsable::parsable;

use crate::{generation::{ARRAY_ALLOC_FUNC_NAME, Wat}, program::{ProgramContext, Type, Wasm}};

use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut all_items_ok = true;
        let mut final_type = Type::Any(0);
        let mut wat = vec![
            Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(self.items.len())])
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

                wat.push(Wat::const_i32(i));
                wat.extend(item_wasm.wat);
                wat.push(Wat::call(final_type.pointer_set_function_name(), vec![]));

                if !item_ok {
                    context.error(item, format!("array literal: incompatible item types `{}` and `{}`", &final_type, &item_wasm.ty));
                }
            }

            all_items_ok &= item_ok;
        }

        match all_items_ok {
            true => Some(Wasm::typed(Type::array(final_type), wat)),
            false => None
        }
    }
}