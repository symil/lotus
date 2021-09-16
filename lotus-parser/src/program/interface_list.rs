use std::rc::Rc;
use crate::utils::Link;
use super::{FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, Type};

#[derive(Debug, Clone)]
pub struct InterfaceList {
    pub list: Vec<Link<InterfaceBlueprint>>
}

impl InterfaceList {
    pub fn new(list: Vec<Link<InterfaceBlueprint>>) -> Self {
        Self { list }
    }

    pub fn get_associated_type_info(&self, name: &str) -> Option<Rc<InterfaceAssociatedTypeInfo>> {
        for interface_wrapped in &self.list {
            let type_info = interface_wrapped.with_ref(|interface_unwrapped| {
                interface_unwrapped.associated_types.get(name)
            });

            if type_info.is_some() {
                return type_info.cloned();
            }
        }

        None
    }

    pub fn get_method(&self, is_static: bool, name: &str) -> Option<Link<FunctionBlueprint>> {
        for interface_wrapped in &self.list {
            if let Some(result) = interface_wrapped.get_method(is_static, name) {
                return Some(result);
            }
        }

        None
    }

    pub fn contains(&self, interface: &Link<InterfaceBlueprint>) -> bool {
        self.list.contains(interface)
    }
}