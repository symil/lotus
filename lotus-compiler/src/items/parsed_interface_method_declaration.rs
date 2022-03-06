use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{FieldKind, ProgramContext, Type, FunctionBlueprint, MethodQualifier, Visibility, Signature, MethodDetails, Vasm, FunctionBody, FieldVisibility}, utils::Link};
use super::{ParsedMethodQualifier, ParsedFunctionSignature, Identifier, ParsedSemicolonToken, set_function_argument_default_values};

#[parsable]
pub struct ParsedInterfaceMethodDeclaration {
    pub qualifier: Option<ParsedMethodQualifier>,
    pub name: Identifier,
    pub signature: ParsedFunctionSignature,
    pub semicolon: Option<ParsedSemicolonToken>,
}

impl ParsedInterfaceMethodDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) -> Link<FunctionBlueprint> {
        let interface = context.get_current_interface().unwrap();
        let qualifier = self.qualifier.as_ref().map(|keyword| keyword.process()).unwrap_or(MethodQualifier::None);
        let (arguments, return_type) = self.signature.process(context);

        let this_type = match qualifier.to_field_kind() {
            FieldKind::Regular => Some(Type::this(interface.clone())),
            FieldKind::Static => None,
        };

        let signature = Signature::create(
            this_type,
            arguments.iter().map(|arg| arg.ty.clone()).collect(),
            return_type.unwrap_or(context.void_type())
        );

        let function_blueprint = FunctionBlueprint {
            name: self.name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            arguments,
            signature,
            argument_variables: vec![],
            owner_type: None,
            owner_interface: Some(interface.clone()),
            closure_details: None,
            method_details: Some(MethodDetails {
                qualifier: qualifier,
                visibility: FieldVisibility::from_name(self.name.as_str()),
                event_callback_details: None,
                first_declared_by: None,
                dynamic_index: None,
                is_autogen: false,
            }),
            body: FunctionBody::Empty
        };

        context.functions.insert(function_blueprint, None)
    }

    pub fn process_default_arguments(&self, context: &mut ProgramContext) {
        let function_wrapped = context.functions.get_by_location(&self.name, None);

        set_function_argument_default_values(&function_wrapped, &self.signature, context);
    }
}