use std::rc::Rc;
use parsable::ItemLocation;
use crate::{items::Identifier, utils::Link};
use super::{InterfaceBlueprint, InterfaceList, Type};

#[derive(Debug)]
pub struct ParameterTypeInfo {
    pub key: ItemLocation,
    pub name: Identifier,
    pub index: usize,
    pub required_interfaces: InterfaceList,
    pub inherited_type: Option<Type>,
    pub wasm_pattern: String
}

impl ParameterTypeInfo {
    pub fn get_id(self: &Rc<Self>) -> u64 {
        Rc::as_ptr(self) as u64
    }
}