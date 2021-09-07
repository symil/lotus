use indexmap::IndexSet;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{FunctionBlueprint, ProgramContext, Type}};
use super::{EventCallbackQualifier, FunctionConditionList, FunctionQualifier, FunctionSignature, Identifier, StatementList, Visibility};

#[parsable]
pub struct FunctionContent {
    pub qualifier: Option<FunctionQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub conditions: Option<FunctionConditionList>,
    pub signature: Option<FunctionSignature>,
    pub statements: StatementList,
}

impl FunctionContent {
    pub fn process_signature(&self, context: &mut ProgramContext) -> FunctionBlueprint {
        let mut function_blueprint = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: self.name.to_string(),
            location: self.location.clone(),
            visibility: Visibility::Private,
            event_callback_qualifier: None,
            generics: IndexSet::new(),
            owner: None,
            this_type: None,
            payload_type: None,
            conditions: vec![],
            arguments: vec![],
            return_type: None,
            body: vec![],
        };

        let is_static = self.qualifier.contains(&FunctionQualifier::Static);

        if let Some(type_id) = context.current_type {
            let type_blueprint = context.types.get_by_id(type_id).unwrap();

            function_blueprint.owner = Some(type_id);
            function_blueprint.generics = type_blueprint.generics.clone();

            if !is_static {
                function_blueprint.this_type = Some(Type::Actual(type_blueprint.get_typeref()));
            }
        } else if is_static {
            context.errors.add(self, "regular functions cannot be static");
        }

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            function_blueprint.arguments = arguments;
            function_blueprint.return_type = return_type;
        }

        if let Some(qualifier) = &self.event_callback_qualifier {
            if let Some(type_id) = context.current_type {
                if let Some(signature) = &self.signature {
                    context.errors.add(signature, "event callbacks do not take arguments nor have a return type");
                }

                if is_static {
                    context.errors.add(self, "event callbacks cannot be static");
                }

                if let Some(event_type) = context.types.get_by_name(&self.name) {
                    function_blueprint.payload_type = Some(Type::Actual(event_type.get_typeref()));

                    if event_type.qualifier != TypeQualifier::Class {
                        context.errors.add(&self.name, format!("type `{}` is not a class", &self.name));
                    } else if let Some(conditions) = &self.conditions {
                        function_blueprint.conditions = conditions.process(event_type.type_id, context);
                    }
                } else {
                    context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
                }
            } else {
                context.errors.add(self, "regular functions cannot be event callbacks");
            }
        } else {
            if self.conditions.is_some() {
                context.errors.add(self, "only event callbacks can have conditions");
            }

            if self.signature.is_none() {
                context.errors.add(&self.name, "missing function signature");
            }
        }

        function_blueprint
    }

    pub fn process_body(&self, function_id: u64, context: &mut ProgramContext) {

    }
}