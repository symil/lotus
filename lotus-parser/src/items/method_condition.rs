use parsable::parsable;
use crate::program::{ProgramContext, Side, Type};

use super::{Identifier, MethodConditionOperand};

#[parsable]
pub struct MethodCondition {
    pub left: MethodConditionOperand,
    #[parsable(prefix="=")]
    pub right: MethodConditionOperand
}

impl MethodCondition {
    pub fn process(&self, struct_name: &Identifier, method_name: &Identifier, context: &mut ProgramContext) {
        self.left.process(struct_name, method_name, Side::Left, context);
        self.right.process(struct_name, method_name, Side::Right, context);
    }
}