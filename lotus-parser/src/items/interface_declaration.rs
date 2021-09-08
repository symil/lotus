use parsable::parsable;
use super::{VisibilityToken, InterfaceQualifier};

#[parsable]
pub struct InterfaceDeclaration {
    pub visibility: VisibilityToken,
    pub qualifier: InterfaceQualifier
}