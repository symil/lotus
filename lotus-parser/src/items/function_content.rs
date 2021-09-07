use parsable::parsable;
use crate::program::{FunctionBlueprint, ProgramContext};
use super::{FunctionConditionList, FunctionSignature, Identifier, FunctionPrefix, StatementList};

#[parsable]
pub struct FunctionContent {
    pub prefix: Option<FunctionPrefix>,
    pub name: Identifier,
    pub conditions: Option<FunctionConditionList>,
    pub signature: FunctionSignature,
    pub statements: StatementList,
}

impl FunctionContent {
    pub fn process_signature(&self, is_static: bool, context: &mut ProgramContext) -> Option<FunctionBlueprint> {

    }

    pub fn process_body(&self, function_id: u64, context: &mut ProgramContext) {

    }
}