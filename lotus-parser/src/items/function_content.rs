use indexmap::IndexSet;
use parsable::parsable;
use crate::{generation::Wat, items::TypeQualifier, program::{FunctionBlueprint, ProgramContext, RESULT_VAR_NAME, ScopeKind, Type, VariableKind}};
use super::{EventCallbackQualifier, FunctionBody, FunctionConditionList, FunctionQualifier, FunctionSignature, Identifier, StatementList, Visibility};

#[parsable]
pub struct FunctionContent {
    pub qualifier: Option<FunctionQualifier>,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub name: Identifier,
    pub conditions: Option<FunctionConditionList>,
    pub signature: Option<FunctionSignature>,
    pub body: FunctionBody,
}

impl FunctionContent {
    pub fn process_signature(&self, context: &mut ProgramContext) -> FunctionBlueprint {
        let mut function_blueprint = FunctionBlueprint {
            function_id: self.location.get_hash(),
            name: self.name.clone(),
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
            is_raw_wasm: false,
            declaration: None,
            call: vec![]
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
        context.current_function = Some(function_id);
        context.reset_local_scope();
        context.push_scope(ScopeKind::Function);

        let is_raw_wasm = self.body.is_raw_wasm();
        let function_blueprint = context.functions.get_by_id(function_id).unwrap();
        let mut wat_args = vec![];
        let mut wat_ret = None;
        let mut wat_locals = vec![];
        let mut wat_body = vec![];

        if let Some(wasm) = self.body.process(context) {
            wat_body.extend(wasm.wat);

            for var_info in wasm.variables {
                if var_info.kind == VariableKind::Local {
                    if let Some(wasm_type) = var_info.ty.get_wasm_type(context) {
                        wat_locals.push((var_info.wasm_name, wasm_type));
                    }
                }
            }
        }

        if !is_raw_wasm {
            for (arg_name, arg_type) in &function_blueprint.arguments {
                let var_info = context.push_var(arg_name, arg_type, VariableKind::Argument);

                if let Some(wasm_type) = arg_type.get_wasm_type(context) {
                    wat_args.push((var_info.wasm_name, wasm_type));
                }
            }

            if let Some(return_type) = &function_blueprint.return_type {
                if let Some(wasm_type) = return_type.get_wasm_type(context) {
                    wat_ret = Some(wasm_type);
                    wat_locals.push((RESULT_VAR_NAME.to_string(), wasm_type));
                    wat_body.push(Wat::get_local(RESULT_VAR_NAME));
                }

                if !context.return_found {
                    context.errors.add(&self.signature.as_ref().unwrap().return_type.as_ref().unwrap(), format!("not all branches return a value for the function"));
                }
            }
        }

        wat_body = vec![Wat::new("block", wat_body)];

        context.pop_scope();
        context.current_function = None;

        let wat_args : Vec<(&str, &str)> = wat_args.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.as_str())).collect();
        let wat_locals : Vec<(&str, &str)> = wat_locals.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.as_str())).collect();

        context.functions.get_mut_by_id(function_id).body = Wat::declare_function(&wasm_func_name, None, wat_args, wat_ret, wat_locals, wat_body);
    }
}