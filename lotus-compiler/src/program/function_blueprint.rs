use std::rc::Rc;
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
    pub method_details: Option<MethodDetails>,
    pub is_raw_wasm: bool,
    pub body: Vasm
}

#[derive(Debug)]
pub struct MethodDetails {
    pub qualifier: Option<MethodQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub first_declared_by: Option<Link<TypeBlueprint>>,
    pub conditions: Vec<(Identifier, Identifier)>,
    pub dynamic_index: i32,
}

impl FunctionBlueprint {
    pub fn is_static(&self) -> bool {
        self.signature.this_type.is_none()
    }

    pub fn get_dynamic_index(&self) -> Option<usize> {
        match &self.method_details {
            Some(details) => match details.dynamic_index > -1 {
                true => Some(details.dynamic_index as usize),
                false => None,
            },
            None => None,
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match &self.method_details {
            Some(details) => details.qualifier.contains(&MethodQualifier::Dynamic),
            None => false,
        }
    }

    pub fn get_method_kind(&self) -> FieldKind {
        match self.is_static() {
            true => FieldKind::Static,
            false => FieldKind::Regular,
        }
    }

    pub fn check_type_parameters(&self, context: &mut ProgramContext) {
        self.signature.check_type_parameters(context);
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}