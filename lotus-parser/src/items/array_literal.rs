use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_BODY_FUNC_NAME, PTR_SET_METHOD_NAME, ProgramContext, Type, TypeOld, VariableInfo, VariableKind, IrFragment}, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let array_var_name = Identifier::unique("array", self);
        let array_body_var_name = Identifier::unique("array_body", self);
        let variables = vec![
            VariableInfo::new(array_var_name.clone(), context.int_type(), VariableKind::Local),
            VariableInfo::new(array_body_var_name.clone(), context.int_type(), VariableKind::Local),
        ];

        let mut all_items_ok = true;
        let mut final_item_type = Type::Any;
        let mut wat = vec![
            Wat::set_local(array_var_name.as_str(), Wat::call(ARRAY_ALLOC_FUNC_NAME, vec![Wat::const_i32(self.items.len())])),
            Wat::set_local(array_body_var_name.as_str(), Wat::call(ARRAY_GET_BODY_FUNC_NAME, vec![Wat::get_local(&array_var_name)]))
        ];

        for (i, item) in self.items.iter().enumerate() {
            let mut item_ok = false;

            if let Some(item_wasm) = item.process(context) {
                if final_item_type.is_assignable_to(&item_wasm.ty) {
                    final_item_type = item_wasm.ty.clone();
                    item_ok = true;
                } else if item_wasm.ty.is_assignable_to(&final_item_type) {
                    item_ok = true;
                }

                wat.extend(item_wasm.wat);
                wat.extend(vec![
                    Wat::get_local(&array_body_var_name),
                    Wat::const_i32(i),
                    final_item_type.method_call_placeholder(PTR_SET_METHOD_NAME)
                ]);

                if !item_ok {
                    context.errors.add(item, format!("incompatible item types `{}` and `{}`", &final_item_type, &item_wasm.ty));
                }
            }

            all_items_ok &= item_ok;
        }

        wat.push(Wat::get_local(&array_var_name));

        match all_items_ok {
            true => Some(IrFragment::new(context.array_type(final_item_type), wat, variables)),
            false => None
        }
    }
}