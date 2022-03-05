use std::{collections::HashSet, rc::Rc};
use indexmap::{IndexMap, IndexSet};
use parsable::ItemLocation;
use crate::{items::{ParsedEventCallbackQualifierKeyword, ParsedMethodQualifier, Identifier, ParsedVisibilityToken}, program::{VariableKind, Wat}, utils::Link};
use super::{FieldKind, FunctionInstanceContent, GlobalItem, InterfaceBlueprint, ParameterTypeInfo, ProgramContext, Signature, Type, TypeBlueprint, TypeIndex, TypeInstanceContent, VariableInfo, Vasm, VirtualInstruction, Visibility, EventCallbackQualifier, MethodQualifier, FunctionBody, FieldVisibility, ArgumentInfo};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub name: Identifier,
    pub visibility: Visibility,
    pub parameters: IndexMap<String, Rc<ParameterTypeInfo>>,
    pub arguments: Vec<ArgumentInfo>,
    pub signature: Signature,
    pub argument_variables: Vec<VariableInfo>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub closure_details: Option<ClosureDetails>,
    pub method_details: Option<MethodDetails>,
    pub body: FunctionBody
}

#[derive(Debug)]
pub struct ClosureDetails {
    pub variables: HashSet<VariableInfo>,
    pub declaration_level: u32,
    pub retain_function: Option<Link<FunctionBlueprint>>,
}

#[derive(Debug)]
pub struct MethodDetails {
    pub qualifier: MethodQualifier,
    pub visibility: FieldVisibility,
    pub event_callback_details: Option<EventCallbackDetails>,
    pub first_declared_by: Option<Link<TypeBlueprint>>,
    pub dynamic_index: Option<i32>,
    pub is_autogen: bool
}

#[derive(Debug)]
pub struct EventCallbackDetails {
    pub event_type: Link<TypeBlueprint>,
    pub qualifier: EventCallbackQualifier,
    pub priority: Vasm,
}

impl FunctionBlueprint {
    pub fn new(name: Identifier, context: &ProgramContext) -> Self {
        Self {
            name: name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            arguments: vec![],
            signature: Signature::void(context),
            argument_variables: vec![],
            owner_type: None,
            owner_interface: None,
            closure_details: None,
            method_details: None,
            body: FunctionBody::Empty
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

    pub fn get_event_callback_details(&self) -> Option<&EventCallbackDetails> {
        match &self.method_details {
            Some(method_details) => match &method_details.event_callback_details {
                Some(details) => Some(details),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_self_type(&self) -> Type {
        Type::function(&self.signature)
    }

    pub fn destroy(&mut self) {
        for arg in &mut self.argument_variables {
            arg.destroy();
        }

        if let Some(closure_details) = &mut self.closure_details {
            if let Some(retain) = &mut closure_details.retain_function {
                retain.borrow_mut().destroy();
            }

            for var_info in &closure_details.variables {
                var_info.destroy();
            }

            closure_details.variables.clear();
        }

        self.parameters.clear();
        self.arguments.clear();
        self.signature = Signature::undefined();
        self.argument_variables.clear();
        self.owner_type = None;
        self.owner_interface = None;
        self.closure_details = None;
        self.method_details = None;
        self.body = FunctionBody::Empty;
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}