use parsable::parsable;

use crate::program::ProgramContext;

use super::FunctionCondition;

#[parsable]
pub struct FunctionConditionList {
    #[parsable(brackets="[]", separator=",")]
    pub list: Vec<FunctionCondition>,
}

impl FunctionConditionList {
    pub fn process(&self, event_type_id: u64, context: &mut ProgramContext) -> Vec<(String, String)> {
        let mut result = vec![];

        for item in &self.list {
            if let Some(fields) = item.process(event_type_id, context) {
                result.push(fields);
            }
        }

        result
    }
}