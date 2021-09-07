use parsable::parsable;
use crate::{generation::{Wat}, items::Visibility, program::{ProgramContext, ScopeKind, TypeOld, VariableKind, Wasm, RESULT_VAR_NAME}};
use super::{FullType, FunctionSignature, Identifier, Statement, StatementList, VisibilityToken};

#[parsable]
pub struct FunctionDeclaration {
    pub visibility: VisibilityToken,
    #[parsable(prefix="fn")]
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub statements: StatementList,
}

impl FunctionDeclaration {
    pub fn process_signature(&self, index: usize, context: &mut ProgramContext) {
        context.set_file_location(&self.file_name, &self.file_namespace);

        let (arguments, return_type) = self.signature.process(context);
        let visibility = self.visibility.get_token();

        if self.name.as_str() == "main" {
            if !arguments.is_empty() {
                context.errors.add(&self.name, format!("main function must not take any argument"));
            }

            if !return_type.is_void() {
                context.errors.add(&self.name, format!("main function must not have a return type"));
            }

            if visibility != Visibility::Export {
                context.errors.add(&self.name, format!("main function must be declared with the `export` visibility"));
            }
        }

        let function_annotation = FunctionAnnotation {
            metadata: ItemMetadata {
                id: index,
                name: self.name.clone(),
                file_name: context.get_current_file_name(),
                file_namespace: context.get_current_file_namespace(),
                visibility: visibility.clone(),
            },
            wasm_name: match visibility {
                Visibility::System => self.name.to_string(),
                _ => format!("{}_{}", self.name, index)
            },
            this_type: None,
            payload_type: None,
            arguments: arguments,
            return_type: return_type,
            wat: Wat::default(),
        };

        if context.get_function_by_name(&self.name).is_some() {
            context.errors.add(&self.name, format!("duplicate function declaration: `{}`", &self.name));
        }
        
        context.add_function(function_annotation);
    }

    pub fn process_body(&self, index: usize, context: &mut ProgramContext) {
        context.set_file_location(&self.file_name, &self.file_namespace);

        let mut ok = true;
        let mut wasm_func_name = String::new();
        let mut wat_args = vec![];
        let mut wat_ret = None;
        let mut wat_locals = vec![];
        let mut wat_body = vec![];

        let mut return_type = None;
        let mut arguments = vec![];

        if let Some(function_annotation) = context.get_function_by_id(index) {
            return_type = match function_annotation.return_type {
                TypeOld::Void => None,
                _ => Some(function_annotation.return_type.clone())
            };
            arguments = function_annotation.arguments.clone();
            wasm_func_name = function_annotation.wasm_name.clone();
            wat_ret = function_annotation.return_type.get_wasm_type();
        }

        context.reset_local_scope();
        context.push_scope(ScopeKind::Function);
        context.set_function_return_type(return_type);

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
            context.errors.add(&self.signature.return_type.as_ref().unwrap(), format!("not all branches return a value for the function"));
            ok = false;
        }

        let wat_args : Vec<(&str, &str)> = wat_args.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();
        let wat_locals : Vec<(&str, &str)> = wat_locals.iter().map(|(arg_name, arg_type)| (arg_name.as_str(), arg_type.clone())).collect();

        if let Some(function_annotation) = context.get_function_by_id_mut(index) {
            function_annotation.wat = Wat::declare_function(&wasm_func_name, None, wat_args, wat_ret, wat_locals, wat_body);
        }

        context.pop_scope();
    }
}