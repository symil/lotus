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
    pub qualifier: MethodQualifier,
    pub parameters: IndexMap<String, Rc<ParameterTypeInfo>>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type: Option<Link<TypeBlueprint>>,
    pub owner_interface: Option<Link<InterfaceBlueprint>>,
    pub first_declared_by: Option<Link<TypeBlueprint>>,
    pub conditions: Vec<(Identifier, Identifier)>,
    pub this_arg: Option<Rc<VariableInfo>>,
    pub payload_arg: Option<Rc<VariableInfo>>,
    pub arguments: Vec<Rc<VariableInfo>>,
    pub return_value: Rc<VariableInfo>,
    pub is_raw_wasm: bool,
    pub dynamic_index: i32,
    pub body: Vasm
}

impl FunctionBlueprint {
    pub fn is_static(&self) -> bool {
        self.this_arg.is_none()
    }

    pub fn is_dynamic(&self) -> bool {
        self.qualifier == MethodQualifier::Dynamic
    }

    pub fn get_method_kind(&self) -> FieldKind {
        match self.is_static() {
            true => FieldKind::Static,
            false => FieldKind::Regular,
        }
    }

    pub fn check_types_parameters(&self, context: &mut ProgramContext) {
        for arg in &self.arguments {
            arg.check_parameters(context);
        }

        self.return_value.check_parameters(context);
    }

    pub fn get_signature(&self) -> Signature {
        Signature {
            this_type: self.this_arg.as_ref().map(|var_info| var_info.ty.clone()),
            argument_types: self.arguments.iter().map(|var_info| var_info.ty.clone()).collect(),
            return_type: self.return_value.ty.clone()
        }
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_name(&self) -> &Identifier { &self.name }
    fn get_visibility(&self) -> Visibility { self.visibility }
}