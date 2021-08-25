use std::fmt::format;

use parsable::parsable;
use crate::{generation::Wat, program::{ARRAY_GET_BODY_FUNC_NAME, ARRAY_GET_ITEM_FUNC_NAME, ARRAY_SET_ITEM_FUNC_NAME, AccessType, ProgramContext, STRING_GET_CHAR_FUNC_NAME, Type, Wasm}, wat};
use super::Expression;

#[parsable]
pub struct BracketIndexing {
    #[parsable(brackets="[]")]
    pub index_expr: Expression
}

impl BracketIndexing {
    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        let mut result = None;
        let mut indexing_ok = false;
        let mut source = vec![];
        let mut index_wasm = Wasm::default();

        if let Some(wasm) = self.index_expr.process(context) {
            if &wasm.ty == &Type::Integer {
                indexing_ok = true;
            } else {
                context.error(&self.index_expr, format!("bracket indexing argument: expected `{}`, got `{}`", Type::Integer, &wasm.ty));
            }

            index_wasm = wasm;
        }

        match parent_type {
            Type::String => {
                match access_type {
                    AccessType::Get => {
                        source.push(index_wasm);
                        source.push(Wasm::simple(Type::Void, Wat::call_from_stack(STRING_GET_CHAR_FUNC_NAME)));
                        result = Some(Wasm::merge(Type::String, source));
                    },
                    AccessType::Set(location) => {
                        context.error(location, format!("strings are immutable"));
                    },
                }
            },
            Type::Pointer(pointed_type) => {
                let func_name = match access_type {
                    AccessType::Get => pointed_type.pointer_get_function_name(),
                    AccessType::Set(_) => pointed_type.pointer_set_function_name(),
                };

                source.push(index_wasm);
                source.push(Wasm::simple(Type::Void, Wat::call(func_name, vec![])));
                result = Some(Wasm::merge(Box::as_ref(pointed_type).clone(), source))
            },
            Type::Array(item_type_boxed) => {
                let item_type = Box::as_ref(item_type_boxed);
                let func_name = match access_type {
                    AccessType::Get => item_type.pointer_get_function_name(),
                    AccessType::Set(_) => item_type.pointer_set_function_name(),
                };

                source.push(Wasm::from_wat(Wat::call_from_stack(ARRAY_GET_BODY_FUNC_NAME)));
                source.push(index_wasm);
                source.push(Wasm::from_wat(Wat::call_from_stack(func_name)));

                result = Some(Wasm::merge(item_type.clone(), source));
            },
            _ => {
                if !parent_type.is_void() {
                    context.error(&self.index_expr, format!("bracket indexing target: expected `{}`, `{}` or `{}`, got `{}`", Type::String, Type::array(Type::Any(0)), Type::pointer(Type::Integer), parent_type));
                }
            }
        }

        match indexing_ok {
            true => result,
            false => None
        }
    }
}