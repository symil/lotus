use std::{rc::Rc, borrow::Borrow};
use indexmap::IndexMap;
use parsable::ItemLocation;
use crate::{items::{Identifier, ParsedVisibilityToken}, utils::Link};
use super::{FuncRef, FunctionBlueprint, GlobalItem, InterfaceList, ParameterTypeInfo, Type, Visibility, FieldKind};

#[derive(Debug)]
pub struct InterfaceBlueprint {
    pub interface_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub associated_types: IndexMap<String, Rc<InterfaceAssociatedTypeInfo>>,
    pub regular_methods: IndexMap<String, FuncRef>,
    pub static_methods: IndexMap<String, FuncRef>,
}

#[derive(Debug)]
pub struct InterfaceAssociatedTypeInfo {
    pub name: Identifier,
    pub required_interfaces: InterfaceList
}

impl Link<InterfaceBlueprint> {
    pub fn get_method(&self, is_static: bool, name: &str) -> Option<FuncRef> {
        self.with_ref(|interface_unwrapped| {
            let index_map = match is_static {
                true => &interface_unwrapped.static_methods,
                false => &interface_unwrapped.regular_methods,
            };

            index_map.get(name).cloned()
        })
    }
}

impl InterfaceBlueprint {
    pub fn methods(&self, kind: FieldKind) -> &IndexMap<String, FuncRef> {
        match kind {
            FieldKind::Regular => &self.regular_methods,
            FieldKind::Static => &self.static_methods,
        }
    }
}

impl GlobalItem for InterfaceBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}