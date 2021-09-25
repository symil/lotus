use std::rc::Rc;
use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{items::{Identifier, Visibility}, utils::Link};
use super::{FunctionBlueprint, GenericTypeInfo, GlobalItem, InterfaceList, Type};

#[derive(Debug)]
pub struct InterfaceBlueprint {
    pub interface_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub associated_types: IndexMap<String, Rc<InterfaceAssociatedTypeInfo>>,
    pub regular_methods: IndexMap<String, Link<FunctionBlueprint>>,
    pub static_methods: IndexMap<String, Link<FunctionBlueprint>>,
}

#[derive(Debug)]
pub struct InterfaceAssociatedTypeInfo {
    pub name: Identifier,
    pub required_interfaces: InterfaceList
}

impl Link<InterfaceBlueprint> {
    pub fn get_method(&self, is_static: bool, name: &str) -> Option<Link<FunctionBlueprint>> {
        self.with_ref(|interface_unwrapped| {
            let index_map = match is_static {
                true => &interface_unwrapped.static_methods,
                false => &interface_unwrapped.regular_methods,
            };

            index_map.get(name).cloned()
        })
    }
}

impl GlobalItem for InterfaceBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}