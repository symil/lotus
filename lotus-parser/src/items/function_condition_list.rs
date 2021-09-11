use parsable::parsable;
use crate::{program::{ProgramContext, TypeBlueprint}, utils::Link};
use super::{FunctionCondition, Identifier};

#[parsable]
pub struct FunctionConditionList {
    #[parsable(brackets="[]", separator=",")]
    pub list: Vec<FunctionCondition>,
}

impl FunctionConditionList {
    pub fn process(&self, event_type_blueprint: &Link<TypeBlueprint>, context: &mut ProgramContext) -> Vec<(Identifier, Identifier)> {
        let mut result = vec![];

        for item in &self.list {
            if let Some(fields) = item.process(event_type_blueprint, context) {
                result.push(fields);
            }
        }

        result
    }
}