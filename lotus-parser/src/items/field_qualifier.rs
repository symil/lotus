use parsable::parsable;

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum FieldQualifier {
    Regular,
    Static = "static"
}