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
                    AccessType::Get => vasm![index_vasm, bracket_vasm],
                    AccessType::Set(_) => {
                        let this_id = self.location.get_hash();
                        let value_id = this_id + 1;
                        let item_type = parent_type.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap();
                        let parent_var = VariableInfo::tmp("parent", parent_type.clone());
                        let item_var = VariableInfo::tmp("item", item_type.clone());

                        let mut result = Vasm::new(Type::Void, vec![parent_var.clone(), item_var.clone()], vec![]);

                        result.extend(vasm![
                            VI::set_tmp_var(&parent_var),
                            VI::set_tmp_var(&item_var),
                            VI::get_tmp_var(&parent_var),
                            index_vasm,
                            VI::get_tmp_var(&item_var),
                            bracket_vasm
                        ]);

                        result
                    },
                });
            }
        }

        result
    }
}