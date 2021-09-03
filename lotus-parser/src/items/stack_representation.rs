use parsable::parsable;
use crate::program::StackType;

#[parsable]
pub struct StackRepresentation {
    pub token: StackRepresentationToken
}

#[parsable]
pub enum StackRepresentationToken {
    Void = "void",
    Int = "i32",
    Float = "f32",
    Pointer = "ptr"
}

impl StackRepresentation {
    pub fn get_stack_type(&self) -> StackType {
        match &self.token {
            StackRepresentationToken::Void => StackType::Void,
            StackRepresentationToken::Int => StackType::Int,
            StackRepresentationToken::Float => StackType::Float,
            StackRepresentationToken::Pointer => StackType::Pointer,
        }
    }
}