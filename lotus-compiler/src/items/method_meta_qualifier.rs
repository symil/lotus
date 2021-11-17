use parsable::parsable;

#[parsable]
#[derive(Clone, Copy, PartialEq)]
pub enum MethodMetaQualifier {
    Autogen = "autogen"
}