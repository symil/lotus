use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{FieldKind, ProgramContext, Type, FunctionBlueprint, MethodQualifier, Visibility, Signature, MethodDetails, Vasm, FunctionBody}, utils::Link};
use super::{ParsedMethodQualifier, ParsedFunctionSignature, Identifier};

#[parsable]
pub struct ParsedInterfaceMethodDeclaration {
    pub qualifier: Option<ParsedMethodQualifier>,
    pub name: Identifier,
    pub signature: ParsedFunctionSignature,
    #[parsable(value=";")]
    pub semicolon: String,
}

impl ParsedInterfaceMethodDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> FunctionBlueprint {
        let interface = context.get_current_interface().unwrap();
        let qualifier = self.qualifier.as_ref().map(|keyword| keyword.process()).unwrap_or(MethodQualifier::None);
        let (arguments, return_type) = self.signature.process(context);

        let this_type = match qualifier.to_field_kind() {
            FieldKind::Regular => Some(Type::this(interface.clone())),
            FieldKind::Static => None,
        };

        FunctionBlueprint {
            name: self.name.clone(),
            visibility: Visibility::None,
            parameters: IndexMap::new(),
            argument_names: arguments.iter().map(|(name, ty)| name.clone()).collect(),
            signature: Signature::create(
                this_type,
                arguments.iter().map(|(name, ty)| ty.clone()).collect(),
                return_type.unwrap_or(context.void_type())
            ),
            argument_variables: vec![],
            owner_type: None,
            owner_interface: Some(interface.clone()),
            closure_details: None,
            method_details: Some(MethodDetails {
                qualifier: qualifier,
                event_callback_details: None,
                first_declared_by: None,
                dynamic_index: None,
            }),
            body: FunctionBody::Empty
        }
    }
}