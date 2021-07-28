use std::{collections::HashMap, ops::Deref};

use parsable::Parsable;

use crate::items::{identifier::Identifier};

use super::{error::Error, expression_type::ExpressionType, function_annotation::FunctionAnnotation, struct_annotation::StructAnnotation};

#[derive(Default)]
pub struct ProgramContext {
    pub errors: Vec<Error>,

    pub structs: HashMap<Identifier, StructAnnotation>,
    pub functions: HashMap<Identifier, FunctionAnnotation>,
    pub constants: HashMap<Identifier, ExpressionType>,
    
    pub scopes: Vec<HashMap<Identifier, ExpressionType>>,
    pub this_type: Option<ExpressionType>,
    pub payload_type: Option<ExpressionType>,
    pub visited_constants: Vec<Identifier>,
    pub inside_const_expr: bool
}

impl ProgramContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn error<T : Parsable, S : Deref<Target=str>>(&mut self, data: &T, error: S) {
        self.errors.push(Error::located(data, error));
    }

    pub fn visit_constant(&mut self, constant_name: &Identifier) -> Option<&Identifier> {
        self.visited_constants.iter().find(|name| *name == constant_name)
    }

    pub fn this(&self) -> Option<&ExpressionType> {
        self.this_type.as_ref()
    }

    pub fn payload(&self) -> Option<&ExpressionType> {
        self.payload_type.as_ref()
    }

    pub fn get_var_type(&self, name: &Identifier) -> Option<&ExpressionType> {
        for scope in self.scopes.iter().rev() {
            if let Some(expr_type) = scope.get(name) {
                return Some(expr_type);
            }
        }

        None
    }
}