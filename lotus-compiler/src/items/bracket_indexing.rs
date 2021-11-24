use parsable::parsable;
use crate::{program::{AccessType, BuiltinInterface, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, Type, VI, VariableInfo, Vasm}, vasm, wat};
use super::Expression;

#[parsable]
pub struct BracketIndexing {
    #[parsable(brackets="[]")]
    pub index_expr: Expression
}

impl BracketIndexing {
    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(index_vasm) = self.index_expr.process(None, context) {
            let required_interface = match access_type {
                AccessType::Get => BuiltinInterface::GetAtIndex,
                AccessType::Set(_) => BuiltinInterface::SetAtIndex,
            };

            if let Some(mut bracket_vasm) = parent_type.call_builtin_interface(self, required_interface, &[(&index_vasm.ty, &self.index_expr)], context, || format!("bracket index")) {
                bracket_vasm.ty = parent_type.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap();

                result = Some(match access_type {
                    AccessType::Get => vasm![
                        index_vasm,
                        bracket_vasm
                    ],
                    AccessType::Set(location) => vasm![
                        index_vasm,
                        VI::placeholder(location),
                        bracket_vasm
                    ],
                });
            }
        }

        result
    }
}