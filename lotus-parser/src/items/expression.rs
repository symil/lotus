use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::{BinaryOperation, FullType, Identifier};

#[parsable]
pub struct Expression {
    pub operation: BinaryOperation,
    #[parsable(prefix="as")]
    pub as_type: Option<FullType>
}

impl Expression {
    pub fn has_side_effects(&self) -> bool {
        self.operation.has_side_effects()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.operation.collected_instancied_type_names(list);
    }

    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = None;

        if let Some(vasm) = self.operation.process(type_hint, context) {
            result = match &self.as_type {
                Some(as_type) => match as_type.process(true, context) {
                    Some(new_type) => Some(Vasm::merge(vec![vasm, Vasm::new(new_type, vec![], vec![])])),
                    None => None
                },
                None => Some(vasm),
            }
        }

        result
    }
}