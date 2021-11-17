use std::rc::Rc;
use crate::{items::Identifier, utils::Link};
use super::{InterfaceBlueprint, InterfaceList, Type, TypeBlueprint};

#[derive(Debug)]
pub struct AssociatedTypeInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
    pub wasm_pattern: String
}

impl AssociatedTypeInfo {
    pub fn get_id(self: &Rc<Self>) -> u64 {
        Rc::as_ptr(self) as u64
    }
}