use std::{fmt::Display};

use parsable::parsable;

use super::{function_declaration::FunctionArgument, identifier::Identifier, statement::Statement};

#[parsable]
pub struct StructDeclaration {
    pub qualifier: StructQualifier,
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub parent: Option<Identifier>,
    #[parsable(brackets="{}")]
    pub body: StructDeclarationBody,
}

#[parsable]
pub struct StructDeclarationBody {
    #[parsable(sep=",")]
    pub fields: Vec<FieldDeclaration>,
    pub methods: Vec<MethodDeclaration>
}

#[parsable]
#[derive(PartialEq, Copy)]
pub enum StructQualifier {
    Struct = "struct",
    View = "view",
    Entity = "entity",
    Event = "event",
    World = "world"
}

// TODO: include this in `parsable` macro
impl Display for StructQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            StructQualifier::Struct => "struct",
            StructQualifier::View => "view",
            StructQualifier::Entity => "entity",
            StructQualifier::Event => "event",
            StructQualifier::World => "world",
        };

        write!(f, "{}", string)
    }
}

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_name: Identifier,
    pub suffix: Option<TypeSuffix>
    // TODO: default value
}

#[parsable]
pub enum TypeSuffix {
    Array = "[]"
}

#[parsable]
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

#[parsable]
pub enum MethodQualifier {
    Builtin = "@",
    Hook = "`",
    Before = "'",
    After= "\""
}

#[parsable]
pub struct MethodCondition {
    pub left: Identifier,
    #[parsable(prefix="=")]
    pub right: Option<Identifier>
}