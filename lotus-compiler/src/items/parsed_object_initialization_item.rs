use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::{ParsedObjectFieldInitialization, ParsedObjectSpreadOperator};

#[parsable]
pub enum ParsedObjectInitializationItem {
    FieldInitialization(ParsedObjectFieldInitialization),
    SpreadOperator(ParsedObjectSpreadOperator)
}

pub struct ObjectInitResult {
    pub init: Option<Vasm>,
    pub fields: Vec<(String, Vasm)>,
}

impl ParsedObjectInitializationItem {
    pub fn process(&self, object_type: &Type, is_last: bool, context: &mut ProgramContext) -> ObjectInitResult {
        match self {
            ParsedObjectInitializationItem::FieldInitialization(field_initialization) => field_initialization.process(object_type, is_last, context),
            ParsedObjectInitializationItem::SpreadOperator(spread_operator) => spread_operator.process(object_type, is_last, context),
        }
    }
}

impl Default for ObjectInitResult {
    fn default() -> Self {
        Self { init: Default::default(), fields: Default::default() }
    }
}