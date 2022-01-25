use std::collections::{HashMap, hash_map::Iter};
use crate::{items::Identifier, program::VariableKind};
use super::{VariableInfo, Type, Wat};

pub struct StringLiteralManager {
    strings: HashMap<String, VariableInfo>,
    counter: usize,
    string_type: Type
}

impl StringLiteralManager {
    pub fn new() -> Self {
        Self {
            strings: HashMap::new(),
            counter: 1,
            string_type: Type::undefined()
        }
    }

    pub fn set_string_type(&mut self, string_type: Type) {
        self.string_type = string_type;
    }

    pub fn add(&mut self, value: &str) -> VariableInfo {
        if let Some(var_info) = self.strings.get(value) {
            return var_info.clone();
        }

        let var_name = format!("string_literal_{}", self.counter);
        let var_info = VariableInfo::create(Identifier::unlocated(&var_name), self.string_type.clone(), VariableKind::Global, u32::MAX);

        self.counter += 1;
        self.strings.insert(value.to_string(), var_info.clone());

        var_info
    }

    pub fn get_all(&self) -> Vec<(String, VariableInfo)> {
        self.strings.iter().map(|(key, value)| (key.to_string(), value.clone())).collect()
    }
}