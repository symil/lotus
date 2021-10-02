use parsable::parsable;
use crate::{program::{AccessType, BuiltinInterface, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, Type, VI, Vasm}, vasm, wat};
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

            if let Some(mut bracket_vasm) = context.call_builtin_interface(self, required_interface, parent_type, &[&index_vasm.ty], || format!("bracket index")) {
                bracket_vasm.ty = parent_type.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap().replace_parameters(Some(parent_type), &[]);

                result = Some(match access_type {
                    AccessType::Get => vasm![index_vasm, bracket_vasm],
                    AccessType::Set(_) => {
                        let this_id = self.location.get_hash();
                        let value_id = this_id + 1;
                        let item_type = parent_type.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap().replace_parameters(Some(parent_type), &[]);

                        vasm![
                            VI::store(parent_type, this_id),
                            VI::store(&item_type, value_id),
                            VI::load(parent_type, this_id),
                            index_vasm,
                            VI::load(&item_type, value_id),
                            bracket_vasm
                        ]
                    },
                });
            }
        }

        result
    }
}