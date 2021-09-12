use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{items::{Identifier, Visibility}, utils::Link};
use super::{GlobalItem, Type};

#[derive(Debug)]
pub struct InterfaceBlueprint {
    pub interface_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub associated_types: IndexMap<String, Link<InterfaceAssociatedType>>,
    pub methods: IndexMap<String, InterfaceMethod>
}

#[derive(Debug)]
pub struct InterfaceAssociatedType {
    pub owner: Link<InterfaceBlueprint>,
    pub name: Identifier
}

#[derive(Debug)]
pub struct InterfaceMethod {
    pub name: Identifier,
    pub arguments: Vec<Type>,
    pub return_type: Option<Type>
}

impl GlobalItem for InterfaceBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}