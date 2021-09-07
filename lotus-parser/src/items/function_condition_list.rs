use parsable::parsable;

use super::FunctionCondition;

#[parsable]
pub struct FunctionConditionList {
    #[parsable(brackets="[]", separator=",")]
    pub list: Vec<FunctionCondition>,
}