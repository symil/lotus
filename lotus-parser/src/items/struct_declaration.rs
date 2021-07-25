use std::{fmt::Display};

use parsable::parsable;

use super::{expr::{VarPath}, function_declaration::FunctionArgument, identifier::Identifier, statement::Statement};

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

#[parsable]
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

// TODO: include this in `parsable` macro
impl Display for StructQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[parsable]
pub struct FieldDeclaration {
    pub name: Identifier,
    #[parsable(prefix=":")]
    pub type_: Type,
    // TODO: default value
}

#[parsable]
pub struct Type {
    pub name: Identifier,
    pub suffix: Option<TypeSuffix>
}

#[parsable]
pub enum TypeSuffix {
    Array = "[]"
}

#[parsable]
pub struct MethodDeclaration {
    pub qualifier: Option<MethodQualifier>,
    pub name: Identifier,
    #[parsable(brackets="[]", separator=",")]
    pub conditions: Vec<MethodCondition>,
    #[parsable(brackets="()", separator=",", optional=true)]
    pub arguments: Vec<FunctionArgument>,
    #[parsable(prefix="->")]
    pub return_type: Option<Type>,
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
    pub left: VarPath,
    #[parsable(prefix="=")]
    pub right: Option<VarPath>
}