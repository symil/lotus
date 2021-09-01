use parsable::parsable;
use crate::{generation::{Wat}, items::VisibilityToken, program::{FunctionAnnotation, ItemMetadata, ProgramContext, ScopeKind, StructInfo, Type, VariableKind, Wasm, display_join, get_builtin_method_info, insert_in_vec_hashmap, RESULT_VAR_NAME, THIS_VAR_NAME}};
use super::{FunctionDeclaration, FunctionSignature, Identifier, MethodCondition, MethodQualifier, Statement, StatementList, StructDeclaration, StructQualifier, VarPath, VarRefPrefix};

#[parsable]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]", separator=",", optional=true)]
    pub conditions: Vec<MethodCondition>,
    pub signature: Option<FunctionSignature>,
    pub statements: StatementList
}

impl MethodDeclaration {
    pub fn process_signature(&self, owner: &StructDeclaration, owner_index: usize, method_index: usize, context: &mut ProgramContext) {
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
            Some(MethodQualifier::Static) | None => {
                if !self.conditions.is_empty() {
                    context.error(&self.conditions[0], format!("only event callbacks can have conditions"));
                }

                if self.signature.is_none() {
                    context.error(&self.name, format!("missing method arguments"));
                }

                if let Some(struct_annotation) = context.get_struct_by_id(owner_index) {
                    let (method_exists, method_this_type) = match &self.qualifier {
                        Some(MethodQualifier::Static) => (struct_annotation.static_methods.contains_key(&self.name), None),
                        None => (struct_annotation.regular_methods.contains_key(&self.name), Some(Type::Struct(struct_annotation.get_struct_info()))),
                        _ => unreachable!()
                    };

                    this_type = method_this_type;

                    if method_exists {
                        context.error(&self.name, format!("duplicate method declaration: method `{}` already exists", &self.name));
                    }
                }
            },
        };

        let mut method_annotation = FunctionAnnotation {
            metadata: ItemMetadata {
                id: method_index,
                name: self.name.clone(),
                file_name: context.get_current_file_name(),
                namespace_name: context.get_current_namespace_name(),
                visibility: VisibilityToken::Private,
            },
            wasm_name: format!("{}_{}_{}_{}", &owner.name, owner_index, &self.name, method_index),
            this_type: this_type,
            payload_type: payload_type,
            arguments: vec![],
            return_type: Type::Void,
            wat: Wat::default(),
        };

        if let Some(signature) = &self.signature {
            let (arguments, return_type) = signature.process(context);

            method_annotation.arguments = arguments;
            method_annotation.return_type = return_type;
        }

        if let Some(struct_annotation) = context.get_struct_by_id_mut(owner_index) {
            match self.qualifier {
                Some(MethodQualifier::Builtin) => struct_annotation.builtin_methods.insert(self.name.clone(), method_annotation),
                Some(MethodQualifier::Hook) => insert_in_vec_hashmap(&mut struct_annotation.hook_event_callbacks, &self.name, method_annotation),
                Some(MethodQualifier::Before) => insert_in_vec_hashmap(&mut struct_annotation.before_event_callbacks, &self.name, method_annotation),
                Some(MethodQualifier::After) => insert_in_vec_hashmap(&mut struct_annotation.after_event_callbacks, &self.name, method_annotation),
                Some(MethodQualifier::Static) => struct_annotation.static_methods.insert(self.name.clone(), method_annotation),
                None => struct_annotation.regular_methods.insert(self.name.clone(), method_annotation),
            };
        }
    }

    pub fn process_body(&self, owner: &StructDeclaration, owner_index: usize, method_index: usize, context: &mut ProgramContext){
        let mut ok = true;
        let mut wasm_func_name = String::new();
        let mut wat_args = vec![];
        let mut wat_ret = None;
        let mut wat_locals = vec![];
        let mut wat_body = vec![];
        let mut return_type = None;
        let mut arguments = vec![];
        let mut this_type = None;
        let mut payload_type = None;
        let mut struct_info = StructInfo::default();
        let is_static = self.qualifier.contains(&MethodQualifier::Static);

        if let Some(struct_annotation) = context.get_struct_by_id(owner_index) {
            struct_info = struct_annotation.get_struct_info();

            let hashmap = match is_static {
                true => &struct_annotation.static_methods,
                false => &struct_annotation.regular_methods
            };

            if let Some(method_annotation) = hashmap.get(&self.name) {
                return_type = match method_annotation.return_type {
                    Type::Void => None,
                    _ => Some(method_annotation.return_type.clone())
                };
                arguments = method_annotation.arguments.clone();
                wasm_func_name = method_annotation.wasm_name.clone();
                wat_ret = method_annotation.return_type.get_wasm_type();
                this_type = method_annotation.this_type.clone();
                payload_type = method_annotation.payload_type.clone();
            }
        }

        if !is_static {
            wat_args.push((THIS_VAR_NAME.to_string(), "i32"));
        }

        context.reset_local_scope();
        context.push_scope(ScopeKind::Function);
        context.set_function_return_type(return_type);
        context.set_this_type(this_type);
        context.set_payload_type(payload_type);

        for (arg_name, arg_type) in &arguments {
            let var_info = context.push_var(arg_name, arg_type, VariableKind::Argument);

            if let Some(wasm_type) = arg_type.get_wasm_type() {
                wat_args.push((var_info.wasm_name, wasm_type));
            } else {
                ok = false;
            }
        }

        if let Some(wasm) = self.statements.process(context) {
            wat_body.extend(wasm.wat);

            for var_info in wasm.variables {
                if var_info.kind == VariableKind::Local {
                    if let Some(wasm_type) = var_info.ty.get_wasm_type() {
                        wat_locals.push((var_info.wasm_name, wasm_type));
                    }
                }
            }
        } else {
            ok = false;
        }

        wat_body = vec![Wat::new("block", wat_body)];

        if let Some(return_type) = &context.function_return_type {
            if let Some(wasm_type) = return_type.get_wasm_type() {
                wat_locals.push((RESULT_VAR_NAME.to_string(), wasm_type));
                wat_body.push(Wat::get_local(RESULT_VAR_NAME));
            }
        }

        if context.function_return_type.is_some() && !context.return_found {
            context.error(&self.signature.as_ref().unwrap().return_type.as_ref().unwrap(), format!("not all branches return a value for the function"));
            ok = false;
        }

        let wat_args : Vec<(&str, &str)> = wat_args.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();
        let wat_locals : Vec<(&str, &str)> = wat_locals.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();

        if let Some(struct_annotation) = context.get_struct_by_id_mut(owner_index) {
            let hashmap = match is_static {
                true => &mut struct_annotation.static_methods,
                false => &mut struct_annotation.regular_methods
            };

            if let Some(method_annotation) = hashmap.get_mut(&self.name) {
                method_annotation.wat = Wat::declare_function(&wasm_func_name, None, wat_args, wat_ret, wat_locals, wat_body);
            }
        }

        context.pop_scope();
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

        if let Some(struct_annotation) = context.get_struct_by_name(&self.name) {
            if struct_annotation.qualifier == StructQualifier::Event {
                ok = true;
            }
        }

        if !ok {
            context.error(self, format!("event callback methods must be named after event names; `{}` is not an event name", &self.name));
        }
    }
}