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
                        context.globals.insert(identifier.clone(), Type::Void);
                        self.const_declarations.insert(identifier, var_declaration);
                    },
                    TopLevelBlock::FunctionDeclaration(def) => {
                        context.functions.insert(identifier.clone(), FunctionAnnotation::new(&identifier));
                        self.function_declarations.insert(identifier, def);
                    },
                }
            }
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
                context.push_local_var(var_name.clone(), VarInfo::const_var(*item_type));
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
}