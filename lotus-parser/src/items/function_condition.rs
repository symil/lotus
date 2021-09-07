use parsable::parsable;
use crate::program::{ProgramContext, Side, TypeOld};
use super::{Identifier, FunctionConditionOperand};

#[parsable]
pub struct FunctionCondition {
    pub left: FunctionConditionOperand,
    #[parsable(prefix="=")]
    pub right: FunctionConditionOperand
}

impl FunctionCondition {
    pub fn process(&self, event_type_id: u64, context: &mut ProgramContext) -> Option<(String, String)> {
        let left = self.left.process(Side::Left, event_type_id ,context);
        let right = self.right.process(Side::Right, event_type_id, context);

        match (left, right) {
            (Some(payload_field_name), Some(this_field_name)) => Some((payload_field_name, this_field_name)),
            _ => None
        }
    }
}