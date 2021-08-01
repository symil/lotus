use parsable::parsable;

use super::{FieldDeclaration, Identifier, MethodDeclaration};

#[parsable]
#[derive(Default)]
pub struct StructDeclaration {
    pub qualifier: StructQualifier,
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub parent: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub body: StructDeclarationBody,
}

#[parsable]
#[derive(Default)]
pub struct StructDeclarationBody {
    #[parsable(sep=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum StructQualifier {
    Struct = "struct",
    View = "view",
    Entity = "entity",
    Event = "event",
    World = "world",
    User = "user",
    Request = "request"
}

impl Default for StructQualifier {
    fn default() -> Self {
        Self::Struct
    }
}


