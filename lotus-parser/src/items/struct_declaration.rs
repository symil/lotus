use parsable::parsable;

use super::{function_declaration::FunctionArgument, identifier::Identifier, statement::Statement};

#[parsable(located)]
pub struct StructDeclaration {
    pub qualifier: StructQualifier,
    pub name: Option<Identifier>,
    #[parsable(prefix="extends")]
    pub extends: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub body: StructDeclarationBody
}

#[parsable(located)]
pub struct StructDeclarationBody {
    #[parsable(sep=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

#[parsable(located)]
pub enum StructQualifier {
    Struct = "struct",
    View = "view",
    Entity = "entity",
    Event = "event",
    World = "world"
}

#[parsable(located)]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub ty: Option<Identifier>
}

#[parsable(located)]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]")]
    pub condition: Option<MethodCondition>,
    #[parsable(brackets="()", separator=",", optional=true)]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub statements: Vec<Statement>
}

#[parsable(located)]
pub enum MethodQualifier {
    Builtin = "@",
    Hook = "`",
    Before = "'",
    After= "\""
}

#[parsable(located)]
pub struct MethodCondition {
    pub left: Identifier,
    #[parsable(prefix="=")]
    pub right: Option<Identifier>
}