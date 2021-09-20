use std::{collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use crate::utils::Link;
use super::{FunctionInstanceContent, TypeBlueprint, TypeInstanceHeader};

#[derive(Debug)]
pub struct TypeInstanceContent {
    // pub associated_types: HashMap<String, Rc<TypeInstanceHeader>>,
}