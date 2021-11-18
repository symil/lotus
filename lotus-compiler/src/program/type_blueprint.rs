use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{Identifier, StackType, TypeQualifier, Visibility}, utils::Link};
use super::{ActualTypeContent, AssociatedTypeInfo, FuncRef, FunctionBlueprint, GlobalItem, InterfaceBlueprint, LOAD_FUNC_NAME, ParameterTypeInfo, ProgramContext, STORE_FUNC_NAME, Type, TypeInstanceContent, TypeInstanceHeader, Vasm};

#[derive(Debug)]
pub struct TypeBlueprint {
    pub declaration_index: usize,
    pub type_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub category: TypeCategory,
    pub stack_type: WasmStackType,
    pub descendants: Vec<Link<TypeBlueprint>>,
    pub ancestors: Vec<Type>,
    pub parameters: IndexMap<String, Rc<ParameterTypeInfo>>,
    pub associated_types: IndexMap<String, Rc<AssociatedTypeInfo>>,
    pub self_type: Type,
    pub parent: Option<ParentInfo>,
    pub enum_variants: IndexMap<String, Rc<EnumVariantInfo>>,
    pub fields: IndexMap<String, Rc<FieldInfo>>,
    pub regular_methods: IndexMap<String, FuncRef>,
    pub static_methods: IndexMap<String, FuncRef>,
    pub dynamic_methods: Vec<FuncRef>,
    pub hook_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub before_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
    pub after_event_callbacks: IndexMap<String, Vec<Link<FunctionBlueprint>>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeCategory {
    Type,
    Enum,
    Class
}

#[derive(Debug)]
pub enum WasmStackType {
    Fixed(StackType),
    TypeParameter(usize)
}

#[derive(Debug)]
pub struct ParentInfo {
    pub location: DataLocation,
    pub ty: Type
}

#[derive(Debug)]
pub struct EnumVariantInfo {
    pub name: Identifier,
    pub value: usize
}

#[derive(Debug)]
pub struct FieldInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
    pub offset: usize,
    pub default_value: Vasm
}

#[derive(Debug)]
pub struct DynamicMethodInfo {
    pub function: Link<FunctionBlueprint>,
    pub this_type: Type,
    pub index: usize
}

impl TypeBlueprint {
    pub fn is_enum(&self) -> bool {
        self.category == TypeCategory::Enum
    }

    pub fn is_class(&self) -> bool {
        self.category == TypeCategory::Class
    }

    pub fn get_wasm_type(&self, parameters: &[Rc<TypeInstanceHeader>]) -> Option<&'static str> {
        match self.stack_type {
            WasmStackType::Fixed(stack_type) => match stack_type {
                StackType::Void => None,
                StackType::Int => Some("i32"),
                StackType::Float => Some("f32"),
            },
            WasmStackType::TypeParameter(index) => parameters[index].wasm_type,
        }
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