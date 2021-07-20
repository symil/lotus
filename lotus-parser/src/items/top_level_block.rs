use parsable::parsable;
use super::{function_declaration::FunctionDeclaration, statement::VarDeclaration, struct_declaration::StructDeclaration};

#[parsable(located)]
pub enum TopLevelBlock {
    StructDeclaration(StructDeclaration),
    VarDeclaration(VarDeclaration),
    FunctionDeclaration(FunctionDeclaration)
}