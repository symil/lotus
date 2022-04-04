use std::rc::Rc;
use parsable::ItemLocation;
use super::TypeInstanceHeader;

pub struct TypeIndex {
    pub current_type_instance: Option<Rc<TypeInstanceHeader>>,
    pub current_function_parameters: Vec<Rc<TypeInstanceHeader>>,
}

impl TypeIndex {
    pub fn get_current_type_parameter(&self, key: &ItemLocation) -> Rc<TypeInstanceHeader> {
        match &self.current_type_instance {
            Some(type_instance) => {
                type_instance.parameters.get(key).unwrap().clone()
            },
            None => unreachable!(),
        }
    }

    pub fn empty() -> Self {
        Self {
            current_type_instance: None,
            current_function_parameters: vec![],
        }
    }
}