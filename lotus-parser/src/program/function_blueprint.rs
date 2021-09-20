use std::rc::Rc;
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{EventCallbackQualifier, Identifier, Visibility}, program::{VariableKind, Wat}, utils::Link};
use super::{FunctionInstanceContent, GenericTypeInfo, GlobalItem, InterfaceBlueprint, ProgramContext, ResolvedType, Type, TypeBlueprint, TypeIndex, TypeInstanceContent, VariableInfo, Vasm, VirtualInstruction};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub function_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub parameters: IndexMap<String, Rc<GenericTypeInfo>>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub conditions: Vec<(Identifier, Identifier)>,
    pub this_arg: Option<Rc<VariableInfo>>,
    pub payload_arg: Option<Rc<VariableInfo>>,
    pub arguments: Vec<Rc<VariableInfo>>,
    pub return_value: Option<Rc<VariableInfo>>,
    pub is_raw_wasm: bool,
    pub is_dynamic: bool,
    pub dynamic_index: i32,
    pub body: Vasm
}

impl FunctionBlueprint {
    pub fn is_static(&self) -> bool {
        self.this_arg.is_none()
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}