use std::collections::HashMap;

use crate::items::{identifier::Identifier};

use super::expression_type::ExpressionType;

pub struct Context {
    this_type: Option<ExpressionType>,
    payload_type: Option<ExpressionType>,
    visited_constants: Vec<Identifier>,
    scopes: Vec<HashMap<Identifier, ExpressionType>>
}

impl Context {
    pub fn new() -> Self {
        Self {
            this_type: None,
            payload_type: None,
            visited_constants: vec![],
            scopes: vec![]
        }
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