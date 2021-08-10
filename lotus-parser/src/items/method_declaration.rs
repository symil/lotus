use parsable::parsable;
use crate::program::{FunctionAnnotation, ProgramContext, Wasm, display_join, get_builtin_method_info, insert_in_vec_hashmap};
use super::{FunctionDeclaration, FunctionSignature, Identifier, MethodCondition, MethodQualifier, Statement, StructDeclaration, StructQualifier, VarPath, VarRefPrefix};

#[parsable]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]", separator=",")]
    pub conditions: Vec<MethodCondition>,
    pub signature: Option<FunctionSignature>,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

impl MethodDeclaration {
    pub fn process_signature(&self, owner: &StructDeclaration, method_index: usize, context: &mut ProgramContext) {
        let mut this_type = None;
        let mut payload_type = None;

        match &self.qualifier {
            Some(MethodQualifier::Builtin) => {
                if let Some((valid_qualifiers, _)) = get_builtin_method_info(&self.name) {
                    if !valid_qualifiers.iter().any(|qualifier| qualifier == &owner.qualifier) {
                        context.error(&self.name, format!("method `@{}` can only be implemented on {}", &self.name, display_join(&valid_qualifiers)));
                    }

                    self.check_self_as_builtin_method(context);
                } else {
                    context.error(self, format!("invalid built-in method name `@{}`", &self.name));
                }
            },
            Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                if !owner.qualifier.is_entity_qualifier() {
                    context.error(self, "event callbacks can only be defined on an entity, world or user");
                }

                self.check_self_as_event_callback(context);

                for condition in &self.conditions {
                    condition.process(&owner.name, &self.name, context);
                }

                if let Some(signature) = &self.signature {
                    context.error(signature, "event callbacks do not take arguments nor have a return type");
                }

                // no need to check for name unicity, multiple event callbacks on the same struct are allowed
            },
            None => {
                if !self.conditions.is_empty() {
                    context.error(&self.conditions[0], format!("only event callbacks can have conditions"));
                }

                if self.signature.is_none() {
                    context.error(&self.name, format!("missing method arguments"));
                }

                if let Some(struct_annotation) = context.structs.get(&owner.name) {
                    // let field_exists = struct_annotation.fields.contains_key(&self.name);
                    let method_exists = struct_annotation.user_methods.contains_key(&self.name);

                    // if field_exists {
                    //     context.error(&self.name, format!("duplicate method declaration: field `{}` already exists", &self.name));
                    // }

                    if method_exists {
                        context.error(&self.name, format!("duplicate method declaration: method `{}` already exists", &self.name));
                    }
                }
            },
        };

        let mut method_annotation = FunctionAnnotation::default();

        method_annotation.wasm_name = format!("method_{}_{}_{}_{}", &owner.name, method_qualifier_to_string(&self.qualifier), &self.name, method_index);
        method_annotation.this_type = this_type;
        method_annotation.payload_type = payload_type;

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            method_annotation.arguments = arguments;
            method_annotation.return_type = return_type;
        }

        if let Some(struct_annotation) = context.structs.get_mut(&owner.name) {
            match self.qualifier {
                Some(MethodQualifier::Builtin) => struct_annotation.builtin_methods.insert(self.name.clone(), method_annotation),
                Some(MethodQualifier::Hook) => insert_in_vec_hashmap(&mut struct_annotation.hook_event_callbacks, &self.name, method_annotation),
                Some(MethodQualifier::Before) => insert_in_vec_hashmap(&mut struct_annotation.before_event_callbacks, &self.name, method_annotation),
                Some(MethodQualifier::After) => insert_in_vec_hashmap(&mut struct_annotation.after_event_callbacks, &self.name, method_annotation),
                None => struct_annotation.user_methods.insert(self.name.clone(), method_annotation),
            };
        }
    }

    pub fn process_body(&self, owner: &StructDeclaration, context: &mut ProgramContext) -> Option<Wasm> {
        todo!()
    }

    fn check_self_as_builtin_method(&self, context: &mut ProgramContext) {
        if !self.conditions.is_empty() {
            context.error(&self.conditions[0], format!("only event callbacks can have conditions"));
        }

        if let Some(signature) = &self.signature {
            context.error(signature, format!("built-in methods do not take arguments nor have a return type"));
        }
    }

    fn check_self_as_event_callback(&self, context: &mut ProgramContext) {
        let mut ok = false;

        if let Some(struct_annotation) = context.structs.get(&self.name) {
            if struct_annotation.qualifier == StructQualifier::Event {
                ok = true;
            }
        }

        if !ok {
            context.error(self, format!("event callback methods must be named after event names; `{}` is not an event name", &self.name));
        }
    }
}

fn method_qualifier_to_string(prefix: &Option<MethodQualifier>) -> &'static str {
    match prefix {
        Some(MethodQualifier::Builtin) => "builtin",
        Some(MethodQualifier::Hook) => "hook",
        Some(MethodQualifier::Before) => "before",
        Some(MethodQualifier::After) => "after",
        None => "user",
    }
}