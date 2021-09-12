use parsable::parsable;
use super::{FunctionDeclaration, GlobalVarDeclaration, InterfaceDeclaration, TypeDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    InterfaceDeclaration(InterfaceDeclaration),
    TypeDeclaration(TypeDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    GlobalDeclaration(GlobalVarDeclaration),
}