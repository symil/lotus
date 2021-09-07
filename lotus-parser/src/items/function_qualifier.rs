use parsable::parsable;

#[parsable]
#[derive(PartialEq)]
pub enum FunctionQualifier {
    Static = "static"
}