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
    pub fn process(&self, struct_name: &Identifier, method_name: &Identifier, context: &mut ProgramContext) {
        self.left.process(struct_name, method_name, Side::Left, context);
        self.right.process(struct_name, method_name, Side::Right, context);
    }
}