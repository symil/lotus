use parsable::parsable;

#[parsable]
#[derive(PartialEq)]
pub enum MethodQualifier {
    Builtin = "@",
    Hook = "`",
    Before = "'",
    After= "\"",
    Static = "static"
}