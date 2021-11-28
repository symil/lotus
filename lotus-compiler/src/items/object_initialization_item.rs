use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{ProgramContext, Type, Vasm};
use super::{ObjectFieldInitialization, SpreadOperator};

#[parsable]
pub enum ObjectInitializationItem {
    FieldInitialization(ObjectFieldInitialization),
    SpreadOperator(SpreadOperator)
}

pub struct ObjectInitResult {
    pub init: Option<Vasm>,
    pub fields: Vec<(String, Vasm)>,
}

impl ObjectInitializationItem {
    pub fn process(&self, object_type: &Type, context: &mut ProgramContext) -> ObjectInitResult {
        match self {
            ObjectInitializationItem::FieldInitialization(field_initialization) => field_initialization.process(object_type, context),
            ObjectInitializationItem::SpreadOperator(spread_operator) => spread_operator.process(object_type, context),
        }
    }
}

impl Default for ObjectInitResult {
    fn default() -> Self {
        Self { init: Default::default(), fields: Default::default() }
    }
}