use std::collections::HashMap;
use parsable::parsable;
use crate::{items::Identifier, program::{BuiltinType, GET_AS_PTR_METHOD_NAME, GET_BODY_FUNC_NAME, NEW_FUNC_NAME, ProgramContext, SET_AT_INDEX_FUNC_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, vasm, wat};
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
        let mut item_vasm_list = vec![];

        for item in self.items.iter() {
            let mut item_ok = false;

            if let Some(item_vasm) = item.process(context) {
                if final_item_type.is_assignable_to(&item_vasm.ty) {
                    final_item_type = item_vasm.ty.clone();
                    item_ok = true;
                } else if item_vasm.ty.is_assignable_to(&final_item_type) {
                    item_ok = true;
                }

                if !item_ok {
                    context.errors.add(item, format!("incompatible item types `{}` and `{}`", &final_item_type, &item_vasm.ty));
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
        let mut result = Vasm::new(Type::Undefined, variables, vec![
            VI::set(&array_var, VI::call_method(&final_array_type, final_array_type.get_static_method(NEW_FUNC_NAME).unwrap(), &[], vec![VI::int(self.items.len())])),
            VI::set(&array_body_var, VI::call_method(&final_array_type, final_array_type.get_method(GET_BODY_FUNC_NAME).unwrap(), &[], vec![VI::get(&array_var)])),
        ]);

        for (i, item_vasm) in item_vasm_list.into_iter().enumerate() {
            result.extend(vasm![
                VI::get(&array_body_var),
                VI::call_method(&final_pointer_type, final_pointer_type.get_method(SET_AT_INDEX_FUNC_NAME).unwrap(), &[], vasm![VI::int(i), item_vasm])
            ]);
        }

        result.extend(VI::get(&array_var));

        Some(result)
    }
}