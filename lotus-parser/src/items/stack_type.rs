use parsable::parsable;

#[parsable]
pub struct StackTypeToken {
    pub value: StackType
}

#[parsable]
#[derive(PartialEq)]
pub enum StackType {
    Void = "void",
    Int = "i32",
    Float = "f32",
    Pointer = "ptr"
}