use std::fmt::format;

use parsable::parsable;
use crate::{generation::Wat, program::{ARRAY_GET_BODY_FUNC_NAME, ARRAY_GET_ITEM_FUNC_NAME, ARRAY_SET_ITEM_FUNC_NAME, AccessType, BuiltinInterface, ProgramContext, STRING_GET_CHAR_FUNC_NAME, Type, Vasm}, wat};
use super::Expression;

#[parsable]
pub struct BracketIndexing {
    #[parsable(brackets="[]")]
    pub index_expr: Expression
}

impl BracketIndexing {
    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(index_wasm) = self.index_expr.process(context) {
            let required_interface = match access_type {
                AccessType::Get => BuiltinInterface::GetAtIndex,
                AccessType::Set(_) => BuiltinInterface::SetAtIndex,
            };

            result = context.call_builtin_interface(self, required_interface, parent_type, &[&index_wasm.ty], || format!("bracket index"));
        }

        result
    }
}