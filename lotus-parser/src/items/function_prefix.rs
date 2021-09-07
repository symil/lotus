use parsable::parsable;

#[parsable]
#[derive(PartialEq)]
pub enum FunctionPrefix {
    Hook = "`",
    Before = "'",
    After= "\"",
}