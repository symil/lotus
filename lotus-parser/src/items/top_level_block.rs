use parsable::parsable;
use super::{function_declaration::FunctionDeclaration, statement::VarDeclaration, struct_declaration::StructDeclaration};

#[parsable]
pub enum TopLevelBlock {
    StructDeclaration(StructDeclaration),
    ConstDeclaration(VarDeclaration),
    FunctionDeclaration(FunctionDeclaration)
}