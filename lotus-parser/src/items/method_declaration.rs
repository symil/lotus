use parsable::parsable;
use crate::{generation::{Wat}, items::Visibility, program::{FunctionBlueprint, MethodDetails, ProgramContext, RESULT_VAR_NAME, ScopeKind, StructInfo, THIS_VAR_NAME, TypeOld, VariableKind, Wasm, display_join, get_builtin_method_info, insert_in_vec_hashmap}};
use super::{EventCallbackQualifier, FunctionCondition, FunctionContent, FunctionDeclaration, FunctionSignature, Identifier, Statement, StatementList, StructDeclaration, TypeQualifier, VarPath, VarRefPrefix};

#[parsable]
pub struct MethodDeclaration {
    pub content: FunctionContent
}

impl MethodDeclaration {
    pub fn process_signature(&self, context: &mut ProgramContext) {
        let type_id = context.current_type.unwrap();
        let mut method_blueprint = self.content.process_signature(context);
        let mut type_blueprint = context.types.get_mut_by_id(type_id);
        let mut index_map = match method_blueprint.is_static() {
            true => &mut type_blueprint.static_methods,
            false => &mut type_blueprint.methods
        };
        let method_details = MethodDetails {
            function_id: method_blueprint.function_id,
            owner_type_id: type_id
        };

        method_blueprint.visibility = Visibility::Member;

        if index_map.insert(method_blueprint.name.clone(), method_details).is_some() {
            let s = match method_blueprint.is_static() {
                true => "static ",
                false => ""
            };
            context.errors.add(self, format!("duplicate {}method `{}`", s, &method_blueprint.name));
        }

        context.functions.insert(method_blueprint);
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
        let is_static = self.qualifier.contains(&EventCallbackQualifier::Static);

        if let Some(struct_annotation) = context.get_struct_by_id(owner_index) {
            struct_info = struct_annotation.get_struct_info();

            let hashmap = match is_static {
                true => &struct_annotation.static_methods,
                false => &struct_annotation.regular_methods
            };

            if let Some(method_annotation) = hashmap.get(&self.name) {
                return_type = match method_annotation.return_type {
                    TypeOld::Void => None,
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
            context.errors.add(&self.signature.as_ref().unwrap().return_type.as_ref().unwrap(), format!("not all branches return a value for the function"));
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
            context.errors.add(&self.conditions[0], format!("only event callbacks can have conditions"));
        }

        if let Some(signature) = &self.signature {
            context.errors.add(signature, format!("built-in methods do not take arguments nor have a return type"));
        }
    }

    fn check_self_as_event_callback(&self, context: &mut ProgramContext) {
        let mut ok = false;

        if let Some(struct_annotation) = context.get_struct_by_name(&self.name) {
            if struct_annotation.qualifier == TypeQualifier::Event {
                ok = true;
            }
        }

        if !ok {
            context.errors.add(self, format!("event callback methods must be named after event names; `{}` is not an event name", &self.name));
        }
    }
}