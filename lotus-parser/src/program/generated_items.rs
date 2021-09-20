use super::{FunctionInstanceContent, FunctionInstanceHeader, GeneratedItemIndex, TypeInstanceContent, TypeInstanceHeader};

#[derive(Debug, Default)]
pub struct GeneratedItems {
    pub type_instances: GeneratedItemIndex<TypeInstanceHeader, TypeInstanceContent>,
    pub function_instances: GeneratedItemIndex<FunctionInstanceHeader, FunctionInstanceContent>,
}