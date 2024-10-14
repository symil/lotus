use crate::utils::Link;
use super::{InterfaceBlueprint, TypeBlueprint};

#[derive(Debug, Clone)]
pub enum TypeOrInterface {
    Type(Link<TypeBlueprint>),
    Interface(Link<InterfaceBlueprint>)
}