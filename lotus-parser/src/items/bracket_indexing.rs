use parsable::parsable;
use crate::{program::{AccessType, BuiltinInterface, ITERABLE_ASSOCIATED_TYPE_NAME, ProgramContext, Type, Vasm}, wat};
use super::Expression;

#[parsable]
pub struct BracketIndexing {
    #[parsable(brackets="[]")]
    pub index_expr: Expression
}

impl BracketIndexing {
    pub fn process(&self, parent_type: &Type, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(index_vasm) = self.index_expr.process(context) {
            let required_interface = match access_type {
                AccessType::Get => BuiltinInterface::GetAtIndex,
                AccessType::Set(_) => BuiltinInterface::SetAtIndex,
            };

            if let Some(mut vasm) = context.call_builtin_interface(self, required_interface, parent_type, &[&index_vasm.ty], || format!("bracket index")) {
                vasm.ty = parent_type.get_associated_type(ITERABLE_ASSOCIATED_TYPE_NAME).unwrap().replace_generics(Some(parent_type), &[]);

                result = Some(Vasm::merge(vec![index_vasm, vasm]));
            }
        }

        result
    }
}