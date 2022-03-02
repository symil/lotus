use std::collections::HashMap;
use super::{MainType, Type, CompilationErrorList};

pub struct MainTypeIndex {
    map: HashMap<MainType, Type>
}

impl MainTypeIndex {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, main_type: MainType, ty: Type) -> Result<(), Type> {
        if let Some(previous_type) = self.map.get(&main_type) {
            if !ty.is_assignable_to(previous_type) {
                return Err(previous_type.clone());
            }
        }

        self.map.insert(main_type, ty);

        Ok(())
    }

    pub fn get(&self, main_type: MainType) -> Option<Type> {
        self.map.get(&main_type).cloned()
    }
}