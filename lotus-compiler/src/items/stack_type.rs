use parsable::parsable;

#[parsable]
#[derive(Clone)]
pub struct StackTypeWrapped {
    pub value: StackType
}

#[parsable]
#[derive(PartialEq, Clone, Copy)]
pub enum StackType {
    Void = "void",
    Int = "i32",
    Float = "f32",
}