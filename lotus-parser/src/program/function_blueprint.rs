use std::rc::Rc;

use indexmap::IndexSet;
use parsable::DataLocation;
use crate::{generation::Wat, items::{EventCallbackQualifier, Identifier, Visibility}, utils::Link};
use super::{FunctionInstance, GlobalItem, ProgramContext, Type, TypeBlueprint, VariableInfo, VirtualInstruction};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub function_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub this_type: Option<Type>,
    pub payload_type: Option<Type>,
    pub conditions: Vec<(Identifier, Identifier)>,
    pub arguments: Vec<Rc<VariableInfo>>,
    pub return_value: Option<Rc<VariableInfo>>,
    pub is_raw_wasm: bool,
    pub body: Vec<VirtualInstruction>
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}