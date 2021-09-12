use parsable::parsable;
use crate::{program::{AccessType, BuiltinInterface, ProgramContext, Type, Vasm}, wat};
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