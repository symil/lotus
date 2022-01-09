use parsable::parsable;

#[parsable]
#[derive(Clone)]
pub struct ParsedStackType {
    pub token: StackTypeToken
}

#[parsable]
#[derive(PartialEq, Clone, Copy)]
pub enum StackTypeToken {
    Void = "void",
    Int = "i32",
    Float = "f32",
}