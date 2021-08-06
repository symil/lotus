use std::{collections::{HashMap, HashSet}};

use crate::{items::{Action, ActionKeyword, ArrayLiteral, Assignment, Branch, Expression, ForBlock, FunctionDeclaration, FunctionSignature, Identifier, IfBlock, LotusFile, MethodDeclaration, MethodQualifier, ObjectLiteral, Operand, Operation, Statement, StructDeclaration, StructQualifier, TopLevelBlock, Type, UnaryOperation, VarDeclaration, VarPath, VarPathRoot, VarPathSegment, VarRef, VarRefPrefix, WhileBlock}, program::{BuiltinMethodPayload, VarInfo, display_join}};

use super::{BuiltinType, Error, ExpressionType, FieldDetails, FunctionAnnotation, ItemType, OperationTree, ProgramContext, StructAnnotation, get_array_field_type, get_binary_operator_input_types, get_binary_operator_output_type, get_builtin_field_type, get_builtin_method_info, get_system_variable_type, get_unary_operator_input_types, get_unary_operator_output_type};

const KEYWORDS : &'static[&'static str] = &[
    "let", "const", "struct", "view", "entity", "event", "world", "user", "true", "false"
];

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
                        context.constants.insert(identifier.clone(), ExpressionType::Void);
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

    fn process_structs_fields(&mut self, context: &mut ProgramContext) {
        for struct_declaration in self.struct_declarations.values() {
            if self.is_forbidden_identifier(&struct_declaration.name) {
                context.error(struct_declaration, format!("invalid type name: {}", &struct_declaration.name));
            } else {
                let struct_types = self.collect_struct_types(&struct_declaration.name, vec![], context);
                let struct_fields = self.collect_struct_fields(&struct_declaration.name, &struct_types, context);
                let struct_annotation = context.structs.get_mut(&struct_declaration.name).unwrap();

                struct_annotation.types = struct_types;
                struct_annotation.fields = struct_fields;
            }
        }
    }

    fn process_structs_methods_signatures(&mut self, context: &mut ProgramContext) {
        for struct_declaration in self.struct_declarations.values() {
            for method in &struct_declaration.body.methods {
                match method.qualifier {
                    Some(MethodQualifier::Builtin) => {
                        if let Some((valid_qualifiers, _)) = get_builtin_method_info(&method.name) {
                            if !valid_qualifiers.iter().any(|qualifier| qualifier == &struct_declaration.qualifier) {
                                context.error(&method.name, format!("method `@{}` can only be implemented on {}", &method.name, display_join(&valid_qualifiers)));
                            }

                            self.check_builtin_method(method, context);
                        } else {
                            context.error(method, format!("invalid built-in method name `@{}`", &method.name));
                        }
                    },
                    Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                        if !self.is_entity_qualifier(&struct_declaration.qualifier) {
                            context.error(method, "event callbacks can only be defined on an entity, world or user");
                        }

                        self.check_struct_qualifier(&method.name, StructQualifier::Event, context);

                        for condition in &method.conditions {
                            if !condition.left.has_payload_prefix() {
                                context.error(&condition.left, "left-hand side of event callback condition must be prefixed by $");
                            }

                            let event_struct_annotation = context.structs.get(&method.name).unwrap();

                            if let Some(field) = event_struct_annotation.fields.get(&condition.left.name) {
                                let mut ok = false;

                                if let ExpressionType::Single(ItemType::Struct(struct_name)) = &field.expr_type {
                                    let field_type = context.structs.get(struct_name).unwrap();

                                    if self.is_entity_qualifier(&field_type.qualifier) {
                                        ok = true;
                                    }
                                }

                                if !ok {
                                    context.error(&condition.left.name, format!("event callback condition: left-side must refer to an entity field"));
                                }
                            } else {
                                context.error(&condition.left.name, format!("event `{}` does not have a `{}` field", &method.name, &condition.left.name));
                            }

                            // if !condition.left.path.is_empty() {
                            //     context.error(&condition.left.path[0], format!("paths are not supported on event callback conditions"));
                            // }

                            if let Some(var_path) = &condition.right {
                                if !var_path.has_this_prefix() {
                                    context.error(&condition.right, "right-hand side of event callback condition must be prefixed by #");
                                }

                                let this_struct_annotation = context.structs.get(&struct_declaration.name).unwrap();

                                if let Some(field) = this_struct_annotation.fields.get(&var_path.name) {
                                    let mut ok = false;

                                    if let ExpressionType::Single(ItemType::Struct(struct_name)) = &field.expr_type {
                                        let field_type = context.structs.get(struct_name).unwrap();

                                        if self.is_entity_qualifier(&field_type.qualifier) {
                                            ok = true;
                                        }
                                    }

                                    if !ok {
                                        context.error(&var_path.name, format!("event callback condition: right-side must refer to an entity field"));
                                    }
                                } else {
                                    context.error(&var_path.name, format!("entity `{}` does not have a `{}` field", &struct_declaration.name, &var_path.name));
                                }

                                // if !var_path.path.is_empty() {
                                //     context.error(&var_path.path[0], format!("paths are not supported on event callback conditions"));
                                // }
                            } else {
                                context.error(&condition.left, "right-hand side of event callback condition must not be empty");
                            }
                        }

                        if let Some(signature) = &method.signature {
                            context.error(signature, "event callbacks do not take arguments nor have a return type");
                        }

                        // no need to check for name unicity, multiple event callbacks on the same struct are allowed
                    },
                    None => {
                        let mut method_annotation = FunctionAnnotation::new(&method.name);

                        if !method.conditions.is_empty() {
                            context.error(&method.conditions[0], format!("only event callbacks can have conditions"));
                        }

                        if let Some(signature) = &method.signature {
                            let (arguments, return_type) = self.process_function_signature(signature, context);

                            method_annotation.arguments = arguments;
                            method_annotation.return_type = return_type;
                        } else {
                            context.error(&method.name, format!("missing method arguments"));
                        }

                        let struct_annotation = context.structs.get_mut(&struct_declaration.name).unwrap();
                        let field_exists = struct_annotation.fields.contains_key(&method.name);
                        let method_exists = struct_annotation.methods.contains_key(&method.name);

                        if !field_exists && !method_exists {
                            struct_annotation.methods.insert(method.name.clone(), method_annotation);
                        }

                        if field_exists {
                            context.error(&method.name, format!("duplicate method declaration: field `{}` already exists", &method.name));
                        }

                        if method_exists {
                            context.error(&method.name, format!("duplicate method declaration: method `{}` already exists", &method.name));
                        }
                    },
                }
            }
        }
    }

    fn process_functions_signatures(&mut self, context: &mut ProgramContext) {
        for function_declaration in self.function_declarations.values() {
            let (arguments, return_type) = self.process_function_signature(&function_declaration.signature, context);
            let function_annotation = context.functions.get_mut(&function_declaration.name).unwrap();

            function_annotation.arguments = arguments;
            function_annotation.return_type = return_type;
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
            global_scope.push((function_name.clone(), function_annotation.get_expr_type()));
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
                    expr_type: ExpressionType::object(&struct_declaration.name),
                    is_const: true
                }));
                context.set_payload_type(match &method_declaration.qualifier {
                    Some(MethodQualifier::Builtin) => match get_builtin_method_info(&method_declaration.name).unwrap().1 {
                        BuiltinMethodPayload::None => None,
                        BuiltinMethodPayload::World => Some(VarInfo::const_var(ExpressionType::object(&self.world_type_name))),
                        BuiltinMethodPayload::User => Some(VarInfo::const_var(ExpressionType::object(&self.user_type_name))),
                        BuiltinMethodPayload::ViewInput => todo!(),
                    },
                    Some(MethodQualifier::Hook | MethodQualifier::Before | MethodQualifier::After) => {
                        Some(VarInfo::const_var(ExpressionType::object(&method_declaration.name)))
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

    fn process_statement(&self, statement: &Statement, context: &mut ProgramContext) {
        match statement {
            Statement::VarDeclaration(var_declaration) => self.process_var_declaration(var_declaration, context),
            Statement::Action(action) => self.process_action(action, context),
            Statement::If(if_block) => self.process_if_block(if_block, context),
            Statement::While(while_block) => self.process_while_block(while_block, context),
            Statement::For(for_block) => self.process_for_block(for_block, context),
            Statement::Assignment(assignment) => self.process_assignment(assignment, context),
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
            if let ExpressionType::Array(item_type) = expr_type {
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
            let valid_condition_type = ExpressionType::builtin(BuiltinType::Boolean);

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

    fn process_function_signature(&self, signature: &FunctionSignature, context: &mut ProgramContext) -> (Vec<(Identifier, ExpressionType)>, ExpressionType) {
        let mut arg_names = HashSet::new();
        let mut arguments = vec![];
        let mut return_type = ExpressionType::Void;

        for argument in &signature.arguments {
            let arg_name = argument.name.clone();

            if !arg_names.insert(arg_name.clone()) {
                context.error(&arg_name, format!("duplicate argument: {}", &arg_name));
            }

            if let Some(arg_type) = self.process_type(&argument.type_, context) {
                arguments.push((arg_name, arg_type));
            } else {
                arguments.push((arg_name, ExpressionType::Void));
            }
        }

        if let Some(ret) = &signature.return_type {
            if let Some(ret_type) = self.process_type(ret, context) {
                return_type = ret_type;
            }
        }

        (arguments, return_type)
    }

    fn is_operand_assignable(&self, operand: &Operand) -> bool {
        match operand {
            Operand::VarPath(var_path) => match var_path.root {
                VarPathRoot::Variable(_) => var_path.path.iter().all(|segment| !segment.is_function_call()),
                _ => false
            },
            _ => false
        }
    }

    fn get_expression_type(&self, expr: &Expression, context: &mut ProgramContext) -> Option<ExpressionType> {
        self.get_operation_type(expr, context)
    }

    fn get_operation_type(&self, operation: &Operation, context: &mut ProgramContext) -> Option<ExpressionType> {
        let operation_tree = OperationTree::from_operation(operation);
        let operation_type = self.get_operation_tree_type(&operation_tree, context);

        match &operation.as_type {
            Some(ty) => self.process_type(ty, context),
            None => operation_type,
        }
    }

    fn get_operation_tree_type(&self, operation_tree: &OperationTree, context: &mut ProgramContext) -> Option<ExpressionType> {
        match operation_tree {
            OperationTree::Operation(left, operator, right) => {
                let left_type = self.get_operation_tree_type(left, context);
                let right_type = self.get_operation_tree_type(right, context);

                match (left_type, right_type) {
                    (Some(ltype), Some(rtype)) => {
                        let operator_valid_types = get_binary_operator_input_types(operator);

                        let left_ok = operator_valid_types.iter().any(|expected| self.expressions_match(expected, &ltype, context));
                        let right_ok = operator_valid_types.iter().any(|expected| self.expressions_match(expected, &rtype, context));
                        let same_type = self.expressions_match(&ltype, &rtype, context);

                        if !left_ok {
                            context.error(left.get_leftmost(), format!("operator `{}`, left operand: expected {}, got `{}`", operator, display_join(&operator_valid_types), ltype));
                        }

                        if !right_ok {
                            context.error(left.get_leftmost(), format!("operator `{}`, right operand: expected {}, got `{}`", operator, display_join(&operator_valid_types), rtype));
                        }

                        if left_ok && right_ok && !same_type {
                            context.error(left.get_leftmost(), format!("operator `{}`: operand types must match (got `{}` and `{}`)", operator, ltype, rtype));
                        }

                        if left_ok && right_ok && same_type {
                            Some(get_binary_operator_output_type(operator, &ltype))
                        } else {
                            None
                        }
                    },
                    _ => None
                }
            },
            OperationTree::Value(operand) => self.get_operand_type(operand, context),
        }
    }

    fn get_operand_type(&self, operand: &Operand, context: &mut ProgramContext) -> Option<ExpressionType> {
        match operand {
            Operand::VoidLiteral => Some(ExpressionType::Void),
            Operand::Parenthesized(expr) => self.get_expression_type(expr, context),
            Operand::UnaryOperation(unary_operation) => self.get_unary_operation_type(unary_operation, context),
            Operand::VarPath(var_path) => self.get_var_path_type(var_path, context),
        }
    }

    fn get_unary_operation_type(&self, unary_operation: &UnaryOperation, context: &mut ProgramContext) -> Option<ExpressionType> {
        let valid_input_types = get_unary_operator_input_types(&unary_operation.operator);

        if let Some(operand_type) = self.get_operand_type(&unary_operation.operand, context) {
            if valid_input_types.iter().any(|expected| self.expressions_match(expected, &operand_type, context)) {
                Some(get_unary_operator_output_type(&unary_operation.operator, &operand_type))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_object_literal_type(&self, object_literal: &ObjectLiteral, context: &mut ProgramContext) -> Option<ExpressionType> {
        let type_name = &object_literal.type_name;

        if let Some(struct_annotation) = context.structs.get(type_name).cloned() { // TODO: remove this `cloned`
            for field_initialization in &object_literal.fields {
                let field_name = &field_initialization.name;
                let field_value = &field_initialization.value;
                let value_type_opt = self.get_expression_type(field_value, context);

                if let Some(field_info) = struct_annotation.fields.get(field_name) {
                    if let Some(actual_type) = value_type_opt {
                        let expected_type = field_info.get_expr_type();

                        if !self.expressions_match(&expected_type, &actual_type, context) {
                            context.error(field_value, format!("field `{}`: expected type `{}`, got `{}`", field_name, expected_type, actual_type));
                        }
                    }
                } else {
                    context.error(field_name, format!("type `{}` has no field `{}`", type_name, field_name));
                }
            }

            Some(ExpressionType::object(type_name))
        } else {
            context.error(type_name, format!("undefined object type `{}`", type_name));
            None
        }
    }

    fn get_array_literal_type(&self, array_literal: &ArrayLiteral, context: &mut ProgramContext) -> Option<ExpressionType> {
        let mut all_items_ok = false;
        let mut final_type = ExpressionType::Any(0);

        for item in &array_literal.items {
            let mut item_ok = false;

            if let Some(item_type) = self.get_expression_type(item, context) {
                if self.expressions_match(&final_type, &item_type, context) {
                    final_type = item_type;
                    item_ok = true;
                }
            }

            all_items_ok &= item_ok;
        }

        match all_items_ok {
            true => Some(ExpressionType::array(final_type)),
            false => None
        }
    }

    fn get_field_access_type(&self, parent_type: &ExpressionType, field_name: &Identifier, context: &mut ProgramContext) -> Option<ExpressionType> {
        let result = match parent_type {
            ExpressionType::Void => None,
            ExpressionType::Single(item_type) => match item_type {
                ItemType::Null => None,
                ItemType::Builtin(builtin_type) => get_builtin_field_type(builtin_type, field_name),
                ItemType::Struct(struct_name) => {
                    if struct_name.is("_") {
                        // special case: `_` refers to the value itself rather than a field
                        // e.g `#foo` means `self.foo`, but `#_` means `self`
                        Some(parent_type.clone())
                    } else if let Some(struct_annotation) = context.structs.get(struct_name) {
                        if let Some(field) = struct_annotation.fields.get(field_name) {
                            Some(field.get_expr_type())
                        } else if let Some(method) = struct_annotation.methods.get(field_name) {
                            Some(method.get_expr_type())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                ItemType::Function(_, _) => None,
            },
            ExpressionType::Array(item_type) => get_array_field_type(item_type, field_name),
            ExpressionType::Any(_) => None,
            
        };

        if result.is_none() {
            context.error(field_name, format!("type `{}` has no field `{}`", parent_type, field_name));
        }

        result
    }

    fn get_path_root_type(&self, path_root: &VarPathRoot, context: &mut ProgramContext) -> Option<ExpressionType> {
        match path_root {
            VarPathRoot::NullLiteral => Some(ExpressionType::Any(0)),
            VarPathRoot::BooleanLiteral(_) => Some(ExpressionType::builtin(BuiltinType::Boolean)),
            VarPathRoot::IntegerLiteral(_) => Some(ExpressionType::builtin(BuiltinType::Integer)),
            VarPathRoot::FloatLiteral(_) => Some(ExpressionType::builtin(BuiltinType::Float)),
            VarPathRoot::StringLiteral(_) => Some(ExpressionType::builtin(BuiltinType::String)),
            VarPathRoot::ArrayLiteral(array_literal) => self.get_array_literal_type(array_literal, context),
            VarPathRoot::ObjectLiteral(object_literal) => self.get_object_literal_type(object_literal, context),
            VarPathRoot::Variable(variable) => self.get_variable_type(variable, context)
        }
    }

    fn get_variable_type(&self, variable: &VarRef, context: &mut ProgramContext) -> Option<ExpressionType> {
        match &variable.prefix {
            Some(prefix) => {
                let prefix_var_opt = match prefix {
                    VarRefPrefix::This => {
                        if context.get_this_type().is_none() {
                            context.error(prefix, "no `this` value can be referenced in this context");
                        }

                        context.get_this_type()
                    },
                    VarRefPrefix::Payload => {
                        if context.get_payload_type().is_none() {
                            context.error(prefix, "no `payload` value can be referenced in this context");
                        }

                        context.get_payload_type()
                    },
                    VarRefPrefix::System => {
                        let result = get_system_variable_type(&variable.name);

                        if result.is_none() {
                            context.error(prefix, format!("undefined system variable `@{}`", &variable.name));
                        }

                        return result;
                    }
                };

                if let Some(prefix_var) = &prefix_var_opt {
                    self.get_field_access_type(&prefix_var.expr_type, &variable.name, context)
                } else {
                    None
                }
            },
            None => {
                if context.inside_const_expr {
                    if let Some(referenced_const) = self.const_declarations.get(&variable.name) {
                        if let Some(_) = context.visit_constant(&variable.name) {
                            context.error(&referenced_const.var_name, format!("circular reference to `{}`", &referenced_const.var_name));

                            None
                        } else {
                            self.get_expression_type(&referenced_const.init_value, context)
                        }
                    } else {
                        context.error(&variable.name, format!("undefined constant `{}`", &variable.name));
                        None
                    }
                } else {
                    context.get_var_type(&variable.name).and_then(|var_info| Some(var_info.expr_type))
                }
            }
        }
    }

    fn get_var_path_type(&self, var_path: &VarPath, context: &mut ProgramContext) -> Option<ExpressionType> {
        let root_type = self.get_path_root_type(&var_path.root, context);

        if context.inside_const_expr && !var_path.path.is_empty() {
            context.error(&var_path.path[0], "field paths are not supported in const expressions");

            None
        } else if let Some(expr_type) = root_type {
            let mut final_type = expr_type.clone();

            for segment in &var_path.path {
                let next_type : Option<ExpressionType> = match segment {
                    VarPathSegment::FieldAccess(field_name) => self.get_field_access_type(&final_type, field_name, context),
                    VarPathSegment::BracketIndexing(expr) => {
                        let array_item_type = match final_type {
                            ExpressionType::Array(item_type) => Some(*item_type),
                            _ => {
                                context.error(expr, format!("bracket indexing target: expected array, got `{}`", final_type)); // TODO: display actual type
                                None
                            }
                        };

                        let indexing_ok = match self.get_expression_type(expr, context) {
                            Some(expr_type) => {
                                if let ExpressionType::Single(ItemType::Builtin(BuiltinType::Integer)) = &expr_type {
                                    true
                                } else {
                                    context.error(expr, format!("bracket indexing argument: expected `{}`, got `{}`", BuiltinType::Integer, &expr_type));
                                    false
                                }
                            },
                            None => false,
                        };

                        match indexing_ok {
                            true => array_item_type,
                            false => None
                        }
                    },
                    VarPathSegment::FunctionCall(arguments) => {
                        match final_type {
                            ExpressionType::Single(ItemType::Function(expected_arguments, return_type)) => {
                                if arguments.as_vec().len() != expected_arguments.len() {
                                    context.error(arguments, format!("function call arguments: expected {} arguments, got `{}`", expected_arguments.len(), arguments.as_vec().len()));
                                }

                                let mut ok = false;
                                let mut anonymous_types = HashMap::new();

                                for (i, (arg_expr, expected_type)) in arguments.as_vec().iter().zip(expected_arguments.iter()).enumerate() {
                                    if let Some(actual_type) = self.get_expression_type(arg_expr, context) {
                                        if !expected_type.match_actual(&actual_type, &context.structs, &mut anonymous_types) {
                                            context.error(arg_expr, format!("function call argument #{}: expected `{}`, got `{}`", i, expected_type, actual_type));
                                            ok = false;
                                        }
                                    }
                                }

                                match ok {
                                    true => Some(*return_type),
                                    false => None
                                }
                            }
                            _ => {
                                context.error(arguments, format!("function call target: expected function, got `{}`", final_type));
                                None
                            }
                        }
                    },
                };

                if let Some(t) = next_type {
                    final_type = t;
                } else {
                    return None;
                }
            }

            Some(final_type)
        } else {
            None
        }
    }

    fn collect_struct_types(&self, struct_name: &Identifier, mut types: Vec<Identifier>, context: &mut ProgramContext) -> Vec<Identifier> {
        if types.contains(struct_name) {
            context.error(struct_name, format!("circular inheritance: {}", struct_name));
        } else {
            types.push(struct_name.clone());

            let struct_def = self.struct_declarations.get(&struct_name).unwrap();

            if let Some(parent_name) = &struct_def.parent {
                if let Some(parent) = self.struct_declarations.get(parent_name) {
                    if parent.qualifier != struct_def.qualifier {
                        context.error(parent_name, format!("a {} cannot inherit from a {}", struct_def.qualifier, parent.qualifier));
                    } else {
                        types = self.collect_struct_types(parent_name, types, context);
                    }
                } else if self.is_builtin_type_name(parent_name) {
                    context.error(parent_name, format!("cannot inherit from built-in type: {}", parent_name))
                } else {
                    context.error(parent_name, format!("unknown type: {}", parent_name))
                }
            }
        }

        types
    }

    fn collect_struct_fields(&self, final_struct_name: &Identifier, struct_types: &[Identifier], context: &mut ProgramContext) -> HashMap<Identifier, FieldDetails> {
        let mut fields = HashMap::new();

        for type_name in struct_types.iter().rev() {
            let struct_declaration = self.struct_declarations.get(type_name).unwrap();

            for field in &struct_declaration.body.fields {
                if self.is_forbidden_identifier(&field.name) {
                    if type_name == final_struct_name {
                        context.error(&field.name, format!("struct `{}`: forbidden field name `{}`", final_struct_name, &field.name));
                    }
                } else {
                    if fields.contains_key(&field.name) {
                        context.error(&field.name, format!("struct `{}`: duplicate field `{}`", final_struct_name, &field.name));
                    }

                    if let Some(field_type) = self.process_type(&field.ty, context) {
                        let ok = match field_type.item_type() {
                            ItemType::Null => false,
                            ItemType::Builtin(_) => true,
                            ItemType::Struct(struct_name) => self.is_entity_qualifier(&self.struct_declarations.get(struct_name).unwrap().qualifier),
                            ItemType::Function(_, _) => false,
                        };

                        if ok {
                            let field_details = FieldDetails {
                                name: field.name.clone(),
                                expr_type: field_type,
                                offset: fields.len(),
                            };

                            fields.insert(field.name.clone(), field_details);
                        } else {
                            context.error(&field.name, format!("field `{}`: expected `bool`, `num`, `string` or an entity, got `{}`", &field.name, &field_type));
                        }
                    }
                }
            }
        }

        fields
    }

    fn check_builtin_method(&self, method: &MethodDeclaration, context: &mut ProgramContext) {
        if !method.conditions.is_empty() {
            context.error(&method.conditions[0], format!("only event callbacks can have conditions"));
        }

        if let Some(signature) = &method.signature {
            context.error(signature, format!("built-in methods do not take arguments nor have a return type"));
        }
    }

    fn is_entity_qualifier(&self, qualifier: &StructQualifier) -> bool {
        match qualifier {
            StructQualifier::Entity | StructQualifier::World | StructQualifier::User => true,
            _ => false
        }
    }

    fn check_struct_qualifier(&self, type_name: &Identifier, required_qualifier: StructQualifier, context: &mut ProgramContext) {
        if self.is_builtin_type_name(type_name) {
            context.error(type_name, format!("required `{}`, got `{}`", required_qualifier, type_name));
        } else if let Some(struct_def) = self.struct_declarations.get(type_name) {
            if struct_def.qualifier != required_qualifier {
                context.error(type_name, format!("required `{}` got `{}`", required_qualifier, type_name));
            }
        } else {
            context.error(type_name, format!("unkown type `{}`", type_name));
        }
    }

    fn is_forbidden_identifier(&self, identifier: &Identifier) -> bool {
        self.is_builtin_type_name(identifier) || KEYWORDS.contains(&identifier.value.as_str())
    }

    fn is_builtin_type_name(&self, name: &Identifier) -> bool {
        BuiltinType::from_identifier(name).is_some()
    }

    fn expressions_match(&self, expected: &ExpressionType, actual: &ExpressionType, context: &ProgramContext) -> bool {
        expected.match_actual(actual, &context.structs, &mut HashMap::new())
    }

    fn process_type(&self, ty: &Type, context: &mut ProgramContext) -> Option<ExpressionType> {
        let mut result = None;

        match ty {
            Type::Value(value_type) => {
                if let Some(expr_type) = ExpressionType::from_value_type(value_type, &self.struct_declarations) {
                    result = Some(expr_type);
                } else {
                    context.error(&value_type.name, format!("undefined type: {}", &value_type.name));
                }
            },
            Type::Function(function_type) => {
                let mut ok = true;
                let mut arguments = vec![];
                let mut return_type = ExpressionType::Void;

                for arg in &function_type.arguments {
                    if let Some(arg_type) = self.process_type(arg, context) {
                        arguments.push(arg_type);
                    } else {
                        arguments.push(ExpressionType::Void);
                        ok = false;
                    }
                }

                if let Some(ret) = &function_type.return_value {
                    if let Some(ret_type) = self.process_type(Box::as_ref(ret), context) {
                        return_type = ret_type;
                    } else {
                        ok = false;
                    }
                }

                if ok {
                    result = Some(ExpressionType::function(arguments, return_type));
                }
            },
        };

        result
    }
}