use indexmap::IndexMap;
use parsable::DataLocation;
use crate::items::{Identifier, Visibility};
use super::{GlobalItem, Type};

#[derive(Debug)]
pub struct InterfaceBlueprint {
    pub interface_id: u64,
    pub name: Identifier,
    pub location: DataLocation,
    pub visibility: Visibility,
    pub associated_types: IndexMap<String, InterfaceAssociatedType>,
    pub methods: IndexMap<String, InterfaceMethod>
}

#[derive(Debug)]
pub struct InterfaceAssociatedType {
    pub name: Identifier
}

#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: Identifier,
    pub arguments: Vec<Type>,
    pub return_type: Option<Type>
}

impl GlobalItem for InterfaceBlueprint {
    fn get_id(&self) -> u64 { self.interface_id }
    fn get_name(&self) -> &str { self.name.as_str() }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> Visibility { self.visibility }
}