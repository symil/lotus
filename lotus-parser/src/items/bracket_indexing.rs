use parsable::parsable;
use crate::{generation::Wat, program::{AccessType, ProgramContext, Type, Wasm}};
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
        let mut wat = vec![];

        if let Some(index_wasm) = self.index_expr.process(context) {
            if &index_wasm.ty == &Type::Integer {
                indexing_ok = true;
            } else {
                context.error(&self.index_expr, format!("bracket indexing argument: expected `{}`, got `{}`", Type::Integer, &index_wasm.ty));
            }

            wat.extend(index_wasm.wat);
        }

        match parent_type {
            Type::Pointer(pointed_type) => {
                let func_name = match access_type {
                    AccessType::Get => pointed_type.pointer_get_function_name(),
                    AccessType::Set(_) => pointed_type.pointer_set_function_name(),
                };

                wat.push(Wat::call(func_name, vec![]));
                result = Some(Wasm::typed(Box::as_ref(pointed_type).clone(), wat))
            },
            Type::Array(item_type) => {
                todo!()
                // let func_name = match access_type {
                //     AccessType::Get => item_type.pointer_get_function_name(),
                //     AccessType::Set(_) => item_type.pointer_set_function_name(),
                // };

                // wat.push(Wat::call(func_name, vec![]));
                // result = Some(Wasm::typed(Box::as_ref(item_type).clone(), wat))
            },
            _ => {
                context.error(&self.index_expr, format!("bracket indexing target: expected `{}` or `{}`, got `{}`", Type::array(Type::Any(0)), Type::pointer(Type::Integer), parent_type));
            }
        }

        match indexing_ok {
            true => result,
            false => None
        }
    }
}