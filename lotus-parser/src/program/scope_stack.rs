use std::collections::HashMap;

use crate::items::{identifier::Identifier, struct_declaration::Type};

pub struct ScopeStack {
    scopes: Vec<HashMap<Identifier, Type>>
}

impl ScopeStack {
    pub fn with_global_scope(global_scope: &HashMap<Identifier, Type>) -> Self {
        Self {
            scopes: vec![global_scope.clone()]
        }
    }
}