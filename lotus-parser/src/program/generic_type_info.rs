use std::rc::Rc;
use crate::{items::Identifier, utils::Link};
use super::{InterfaceBlueprint, InterfaceList, Type};

#[derive(Debug)]
pub struct GenericTypeInfo {
    pub name: Identifier,
    pub index: usize,
    pub required_interfaces: InterfaceList
}

impl GenericTypeInfo {
    pub fn get_id(self: &Rc<Self>) -> u64 {
        Rc::as_ptr(self) as u64
    }
}