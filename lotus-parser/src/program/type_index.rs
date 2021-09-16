use std::rc::Rc;
use indexmap::IndexMap;
use super::TypeInstance;

pub struct TypeIndex<'a, 'b, 'c> {
    pub this_type: &'a TypeInstance,
    pub function_parameters: &'b[&'c TypeInstance]
}