use std::{collections::{HashMap, HashSet}};

use crate::{items::{Action, ActionKeyword, ArrayLiteral, Assignment, Branch, Expression, ForBlock, FunctionDeclaration, FunctionSignature, Identifier, IfBlock, LotusFile, MethodDeclaration, MethodQualifier, ObjectLiteral, Operand, BinaryOperation, Statement, StructDeclaration, StructQualifier, TopLevelBlock, FullType, UnaryOperation, VarDeclaration, VarPath, VarPathRoot, VarPathSegment, VarRef, VarRefPrefix, WhileBlock}, program::{BuiltinMethodPayload, VarInfo, display_join}};

use super::{BuiltinType, Error, Type, FieldDetails, FunctionAnnotation, ItemType, OperationTree, ProgramContext, StructAnnotation, process_array_method_call, get_binary_operator_input_types, get_binary_operator_output_type, process_builtin_field_access, get_builtin_method_info, process_system_variable, get_unary_operator_input_types, get_unary_operator_output_type};

#[derive(Default)]
pub struct ProgramIndex {
    pub world_type_name: Identifier,
    pub user_type_name: Identifier,
    pub struct_declarations: HashMap<Identifier, StructDeclaration>,
    pub function_declarations: HashMap<Identifier, FunctionDeclaration>,
    pub const_declarations: HashMap<Identifier, VarDeclaration>,
}

impl ProgramIndex {
    pub fn from_parsed_files(files: Vec<LotusFile>) -> Result<Self, Vec<Error>> {
        let mut index = Self::default();
        let mut context = ProgramContext::new();

        index.build_index(files, &mut context);
        index.process_structs_fields(&mut context);
        index.process_structs_methods_signatures(&mut context);
        index.process_functions_signatures(&mut context);
        index.process_constants(&mut context);
        index.process_function_and_method_bodies(&mut context);

        match context.errors.is_empty() {
            true => Ok(index),
            false => Err(context.errors)
        }
    }

    fn build_index(&mut self, files: Vec<LotusFile>, context: &mut ProgramContext) {
        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => &const_declaration.var_name,
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.struct_declarations.contains_key(&identifier) || self.function_declarations.contains_key(&identifier) || self.const_declarations.contains_key(&identifier) {
                    context.error(&identifier, format!("duplicate declaration: {}", identifier));
                }

                match block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => {
                        context.structs.insert(identifier.clone(), StructAnnotation::new(&identifier, &struct_declaration.qualifier));
                        self.struct_declarations.insert(identifier, struct_declaration);
                    },
                    TopLevelBlock::ConstDeclaration(var_declaration) => {
                        context.constants.insert(identifier.clone(), Type::Void);
                        self.const_declarations.insert(identifier, var_declaration);
                    },
                    TopLevelBlock::FunctionDeclaration(def) => {
                        context.functions.insert(identifier.clone(), FunctionAnnotation::new(&identifier));
                        self.function_declarations.insert(identifier, def);
                    },
                }
            }
        }

        let world_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::World).map(|s| s.name.clone()).collect();
        let user_structs : Vec<Identifier> = self.struct_declarations.values().filter(|s| s.qualifier == StructQualifier::User).map(|s| s.name.clone()).collect();

        if world_structs.len() > 1 {
            for name in &world_structs {
                context.error(name, "multiple world structures declared");
            }

            self.world_type_name = world_structs.first().unwrap().clone();
        } else if world_structs.is_empty() {
            let mut default_world_struct = StructDeclaration::default();

            default_world_struct.qualifier = StructQualifier::World;
            default_world_struct.name = Identifier::new("__DefaultWorld");

            self.world_type_name = default_world_struct.name.clone();
            self.struct_declarations.insert(default_world_struct.name.clone(), default_world_struct);
        }

        if user_structs.len() > 1 {
            for name in &world_structs {
                context.error(name, "multiple user structures declared");
            }

            self.user_type_name = user_structs.first().unwrap().clone();
        } else if user_structs.is_empty() {
            let mut default_user_struct = StructDeclaration::default();

            default_user_struct.qualifier = StructQualifier::World;
            default_user_struct.name = Identifier::new("__DefaultUser");

            self.user_type_name = default_user_struct.name.clone();
            self.struct_declarations.insert(default_user_struct.name.clone(), default_user_struct);
        }
    }

    fn process_constants(&mut self, context: &mut ProgramContext) {
        context.inside_const_expr = true;

        for const_declaration in self.const_declarations.values() {
            if const_declaration.qualifier.is_none() {
                context.error(const_declaration, "global variables must be declared with the `const` qualifier");
            }

            if let Some(expr_type) = self.get_expression_type(&const_declaration.init_value, context) {
                *context.constants.get_mut(&const_declaration.var_name).unwrap() = expr_type;
            }
        }
    }

    fn process_function_and_method_bodies(&mut self, context: &mut ProgramContext) {
        context.inside_const_expr = false;
        context.push_scope();

        let mut global_scope = vec![];

        for (const_name, const_type) in context.constants.iter() {
            global_scope.push((const_name.clone(), const_type.clone()));
        }

        for (function_name, function_annotation) in context.functions.iter() {
            global_scope.push((function_name.clone(), function_annotation.get_type()));
        }

        for (value_name, value_type) in global_scope {
            context.push_var(value_name, VarInfo {
                expr_type: value_type,
                is_const: true
            });
        }

        for struct_declaration in self.struct_declarations.values() {
            for method_declaration in &struct_declaration.body.methods {
                context.function_return_type = None;
                context.set_this_type(Some(VarInfo {
                    expr_type: Type::object(&struct_declaration.name),
                    is_const: true
                }));
                context.set_payload_type(match &method_declaration.qualifier {
                    Some(MethodQualifier::Builtin) => match get_builtin_method_info(&method_declaration.name).unwrap().1 {
                        BuiltinMethodPayload::None => None,
                        BuiltinMethodPayload::World => Some(VarInfo::const_var(Type::object(&self.world_type_name))),
                        BuiltinMethodPayload::User => Some(VarInfo::const_var(Type::object(&self.user_type_name))),
                        BuiltinMethodPayload::ViewInput => todo!(),
                    },
                    Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                        Some(VarInfo::const_var(Type::object(&method_declaration.name)))
                    },
                    None => None,
                });
                context.push_scope();

                if let Some((arguments, return_type)) = context.get_method_signature(&struct_declaration.name, &method_declaration.name) {
                    context.function_return_type = Some(return_type);

                    for (arg_name, arg_type) in arguments {
                        context.push_var(arg_name, VarInfo::mut_var(arg_type));
                    }
                }

                self.process_function_body(&method_declaration.statements, context);

                context.pop_scope();
            }
        }

        for function_declaration in self.function_declarations.values() {
            context.function_return_type = None;
            context.set_this_type(None);
            context.set_payload_type(None);
            context.push_scope();

            if let Some((arguments, return_type)) = context.get_function_signatures(&function_declaration.name) {
                context.function_return_type = Some(return_type);

                for (arg_name, arg_type) in arguments {
                    context.push_var(arg_name, VarInfo::mut_var(arg_type));
                }
            }

            self.process_function_body(&function_declaration.statements, context);

            context.pop_scope();
        }

        context.push_scope();
    }

    fn process_function_body(&self, body: &Vec<Statement>, context: &mut ProgramContext){ 
        for statement in body {
            self.process_statement(statement, context);
        }
    }

    fn process_for_block(&self, for_block: &ForBlock, context: &mut ProgramContext) {        
        let var_name = &for_block.var_name;
        let var_exists = context.var_exists(var_name);

        if var_exists {
            context.error(var_name, format!("for block: variable `{}` already exists in this scope", var_name));
        }

        context.push_scope();

        if let Some(expr_type) = self.get_expression_type(&for_block.array_expression, context) {
            if let Type::Array(item_type) = expr_type {
                context.push_var(var_name.clone(), VarInfo::const_var(*item_type));
            } else {
                context.error(&for_block.array_expression, format!("for block range: expected array, for `{}`", expr_type));
            }
        }

        for statement in &for_block.statements {
            self.process_statement(statement, context);
        }

        context.pop_scope();
    }

    fn process_while_block(&self, while_block: &WhileBlock, context: &mut ProgramContext) {
        self.process_branch(&while_block.while_branch, context);
    }

    fn process_if_block(&self, if_block: &IfBlock, context: &mut ProgramContext) {
        self.process_branch(&if_block.if_branch, context);

        for branch in &if_block.else_if_branches {
            self.process_branch(branch, context);
        }

        if let Some(else_branch) = &if_block.else_branch {
            self.process_branch(else_branch, context);
        }
    }

    fn process_branch(&self, branch: &Branch, context: &mut ProgramContext) {
        if let Some(condition_type) = self.get_expression_type(&branch.condition, context) {
            let valid_condition_type = Type::builtin(BuiltinType::Boolean);

            if !self.expressions_match(&valid_condition_type, &condition_type, context) {
                context.error(&branch.condition, format!("branch condition: expected `{}`, got `{}`", &valid_condition_type, &condition_type));
            }
        }

        context.push_scope();

        for statement in &branch.statements {
            self.process_statement(statement, context);
        }

        context.pop_scope();
    }

    fn process_action(&self, action: &Action, context: &mut ProgramContext) {
        match &action.keyword {
            ActionKeyword::Return => {
                if let Some(expected_return_type) = context.get_return_type() {
                    if let Some(actual_return_type) = self.get_expression_type(&action.value, context) {
                        if !self.expressions_match(&expected_return_type, &actual_return_type, context) {
                            context.error(&action.value, format!("return statement: expected `{}`, got `{}`", expected_return_type, actual_return_type));
                        }
                    }
                } else {
                    context.error(action, format!("`return` statement not allowed in this context"));
                }
            },
        }
    }

    fn process_var_declaration(&self, var_declaration: &VarDeclaration, context: &mut ProgramContext) {
        let var_name = &var_declaration.var_name;
        let var_exists = context.var_exists(&var_declaration.var_name);

        if var_declaration.qualifier.is_some() {
            context.error(&var_declaration.qualifier, format!("local variables cannot have the `const` qualifier"));
        }

        if var_exists {
            context.error(var_name, format!("duplicate variable declaration: `{}` already exists in this scope", var_name));
        }

        if let Some(var_type) = self.get_expression_type(&var_declaration.init_value, context) {
            if !var_exists {
                context.push_var(var_name.clone(), VarInfo::mut_var(var_type));
            }
        }
    }

    fn process_assignment(&self, assignment: &Assignment, context: &mut ProgramContext) {
        let lvalue = &assignment.lvalue;
        let lvalue_type_opt = self.get_operand_type(lvalue, context);

        if let Some(rvalue) = &assignment.rvalue {
            let is_lvalue_assignable = self.is_operand_assignable(lvalue);

            if !is_lvalue_assignable {
                context.error(lvalue, format!("assignment: invalid left-hand side"));
            }

            if let Some(rvalue_type) = self.get_expression_type(rvalue, context) {
                if let Some(lvalue_type) = lvalue_type_opt {
                    if is_lvalue_assignable {
                        if !self.expressions_match(&lvalue_type, &rvalue_type, context) {
                            context.error(rvalue, format!("assignment: right-hand side type `{}` does not match left-hand side type `{}`", rvalue_type, lvalue_type));
                        }
                    }
                }
            }
        }
    }

}