use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::ItemLocation;
use crate::{items::{ParsedEventCallbackQualifierKeyword, Identifier, ParsedTypeQualifier, ParsedVisibilityToken}, utils::Link};
use super::{ActualTypeContent, AssociatedTypeInfo, FuncRef, FunctionBlueprint, GlobalItem, InterfaceBlueprint, LOAD_FUNC_NAME, ParameterTypeInfo, ProgramContext, STORE_FUNC_NAME, Type, TypeInstanceContent, TypeInstanceHeader, Vasm, Visibility, FieldKind, FieldVisibility};

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
    pub event_callbacks: HashMap<Link<TypeBlueprint>, Vec<Link<FunctionBlueprint>>>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeCategory {
    Type,
    Enum,
    Class
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WasmStackType {
    I32,
    F32,
    Void
}

#[derive(Debug)]
pub struct ParentInfo {
    pub location: ItemLocation,
    pub ty: Type
}

#[derive(Debug)]
pub struct EnumVariantInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub value: usize,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub owner: Link<TypeBlueprint>,
    pub name: Identifier,
    pub ty: Type,
    pub visibility: FieldVisibility,
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
            WasmStackType::I32 => Some("i32"),
            WasmStackType::F32 => Some("f32"),
            WasmStackType::Void => None,
        }
    }

    pub fn check_types_parameters(&self, context: &mut ProgramContext) {
        if let Some(parent) = &self.parent {
            parent.ty.check_parameters(context);
        }

        for field_info in self.fields.values() {
            if field_info.owner.borrow().type_id == self.type_id {
                field_info.ty.check_parameters(context);
            }
        }

        for type_info in self.associated_types.values() {
            if type_info.owner.borrow().type_id == self.type_id {
                type_info.ty.check_parameters(context);
            }
        }
    }

    pub fn methods(&self, kind: FieldKind) -> &IndexMap<String, FuncRef> {
        match kind {
            FieldKind::Regular => &self.regular_methods,
            FieldKind::Static => &self.static_methods,
        }
    }

    pub fn destroy(&mut self) {
        self.descendants.clear();
        self.ancestors.clear();
        self.parameters.clear();
        self.associated_types.clear();
        self.self_type = Type::undefined();
        self.parent = None;
        self.enum_variants.clear();
        self.fields.clear();
        self.regular_methods.clear();
        self.static_methods.clear();
        self.dynamic_methods.clear();
        self.event_callbacks.clear();
    }
}

impl GlobalItem for TypeBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}