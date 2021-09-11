use std::collections::HashMap;
use parsable::parsable;
use crate::{generation::{Wat}, items::Identifier, program::{ARRAY_ALLOC_FUNC_NAME, ARRAY_GET_BODY_FUNC_NAME, PTR_SET_METHOD_NAME, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}, wat};
use super::Expression;

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

impl ArrayLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let array_var = VariableInfo::new(Identifier::unique("array", self), context.int_type(), VariableKind::Local);
        let array_body_var = VariableInfo::new(Identifier::unique("array_body", self), context.int_type(), VariableKind::Local);
        let variables = vec![
            array_var.clone(),
            array_body_var.clone()
        ];

        let mut all_items_ok = true;
        let mut final_item_type = Type::Any;

        let mut content = vec![
            VI::int(self.items.len()),
            VI::static_method(&context.array_type(final_item_type.clone()), "new"),
            VI::set(&array_var),

            VI::get(&array_var),
            VI::method(&context.array_type(final_item_type.clone()), "get_body"),
            VI::set(&array_body_var),
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

                content.extend(item_wasm.wat);
                content.extend(vec![
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

        content.push(Wat::get_local(&array_var_name));

        match all_items_ok {
            true => Some(IrFragment::new(context.array_type(final_item_type), content, variables)),
            false => None
        }
    }
}