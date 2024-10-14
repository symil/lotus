use std::collections::{HashMap, hash_map::Iter};
use crate::{items::Identifier, program::VariableKind};
use super::{VariableInfo, Type, Wat};

pub struct LiteralItemManager {
    strings: HashMap<String, VariableInfo>,
    counter: usize,
    item_name: &'static str,
    item_type: Type
}

impl LiteralItemManager {
    pub fn new(item_name: &'static str) -> Self {
        Self {
            strings: HashMap::new(),
            counter: 1,
            item_name,
            item_type: Type::undefined()
        }
    }

    pub fn set_item_type(&mut self, item_type: Type) {
        self.item_type = item_type;
    }

    pub fn add(&mut self, value: &str) -> VariableInfo {
        if let Some(var_info) = self.strings.get(value) {
            return var_info.clone();
        }

        let var_name = format!("literal_{}_{}", self.item_name, self.counter);
        let var_info = VariableInfo::create(Identifier::unlocated(&var_name), self.item_type.clone(), VariableKind::Global, u32::MAX, None);

        self.counter += 1;
        self.strings.insert(value.to_string(), var_info.clone());

        var_info
    }

    pub fn get_all(&self) -> Vec<(String, VariableInfo)> {
        self.strings.iter().map(|(key, value)| (key.to_string(), value.clone())).collect()
    }
}