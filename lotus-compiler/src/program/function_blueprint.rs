use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::DataLocation;
use crate::{items::{EventCallbackQualifier, MethodQualifier, Identifier, Visibility}, program::{VariableKind, Wat}, utils::Link};
use super::{FieldKind, FunctionInstanceContent, GlobalItem, InterfaceBlueprint, ParameterTypeInfo, ProgramContext, Signature, Type, TypeBlueprint, TypeIndex, TypeInstanceContent, VariableInfo, Vasm, VirtualInstruction};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub function_id: u64,
    pub name: Identifier,
    pub visibility: Visibility,
    pub parameters: IndexMap<String, Rc<ParameterTypeInfo>>,
    pub argument_names: Vec<Identifier>,
    pub signature: Signature,
    pub argument_variables: Vec<VariableInfo>,
    pub closure_details: Option<ClosureDetails>,
    pub method_details: Option<MethodDetails>,
    pub is_raw_wasm: bool,
    pub body: Vasm
}

#[derive(Debug)]
pub struct ClosureDetails {
    pub variables: HashSet<VariableInfo>,
    pub declaration_level: u32,
    pub retain_function: Option<Link<FunctionBlueprint>>,
}

#[derive(Debug)]
pub struct MethodDetails {
    pub event_callback_details: Option<(EventCallbackQualifier, Link<TypeBlueprint>)>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub first_declared_by: Option<Link<TypeBlueprint>>,
    pub dynamic_index: Option<i32>,
}

impl FunctionBlueprint {
    pub fn new(name: Identifier) -> Self {
        Self {
            function_id: name.location.get_hash(),
            name: name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            argument_names: vec!{},
            signature: Signature::default(),
            argument_variables: vec![],
            closure_details: None,
            method_details: None,
            is_raw_wasm: false,
            body: Vasm::void(),
        }
    }
    
    pub fn is_static(&self) -> bool {
        self.signature.this_type.is_none()
    }

    pub fn is_event_callback(&self) -> bool {
        match &self.method_details {
            Some(details) => details.event_callback_details.is_some(),
            None => false,
        }
    }

    pub fn get_dynamic_index(&self) -> Option<usize> {
        match &self.method_details {
            Some(details) => match details.dynamic_index {
                Some(i) => Some(i as usize),
                None => None,
            },
            None => None,
        }
    }

    pub fn is_dynamic(&self) -> bool {
        self.get_dynamic_index().is_some()
    }

    pub fn get_method_kind(&self) -> FieldKind {
        match self.is_static() {
            true => FieldKind::Static,
            false => FieldKind::Regular,
        }
    }

    pub fn is_closure(&self) -> bool {
        self.closure_details.is_some()
    }

    pub fn check_type_parameters(&self, context: &mut ProgramContext) {
        self.signature.check_type_parameters(context);
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}