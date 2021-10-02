use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{Identifier, StackType, TypeQualifier, Visibility}, utils::Link};
use super::{ActualTypeContent, AssociatedTypeInfo, FuncRef, FunctionBlueprint, GlobalItem, InterfaceBlueprint, LOAD_FUNC_NAME, ParameterTypeInfo, ProgramContext, ResolvedType, STORE_FUNC_NAME, Type, TypeInstanceContent};

#[derive(Debug)]
pub struct TypeBlueprint {
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub qualifier: TypeQualifier,
    pub stack_type: StackType,
    pub parameters: IndexMap<String, Rc<ParameterTypeInfo>>,
    pub associated_types: IndexMap<String, Rc<AssociatedTypeInfo>>,
    pub self_type: Type,
    pub parent: Option<ParentInfo>,
    pub fields: IndexMap<String, Rc<FieldInfo>>,
    pub regular_methods: IndexMap<String, FuncRef>,
    pub static_methods: IndexMap<String, FuncRef>,
    pub dynamic_methods: Vec<Link<FunctionBlueprint>>,
    pub hook_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
}

#[derive(Debug)]
pub struct ParentInfo {
    pub location: DataLocation,
    pub ty: Type
}

#[derive(Debug)]
pub struct FieldInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
    pub offset: usize
}

impl TypeBlueprint {
    pub fn get_wasm_type(&self) -> Option<&'static str> {
        match self.stack_type {
            StackType::Void => None,
            StackType::Int => Some("i32"),
            StackType::Float => Some("f32"),
        }
    }

    pub fn generate_builtin_methods(&mut self) {
        let methods = &[
            (STORE_FUNC_NAME, "store"),
            (LOAD_FUNC_NAME, "load")
        ];
    }

    pub fn check_types_parameters(&self, context: &mut ProgramContext) {
        if let Some(parent) = &self.parent {
            parent.ty.check_parameters(&parent.location, context);
        }

        for field_info in self.fields.values() {
            if field_info.owner.borrow().type_id == self.type_id {
                field_info.ty.check_parameters(&field_info.name, context);
            }
        }

        for type_info in self.associated_types.values() {
            if type_info.owner.borrow().type_id == self.type_id {
                type_info.ty.check_parameters(&type_info.name, context);
            }
        }
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}