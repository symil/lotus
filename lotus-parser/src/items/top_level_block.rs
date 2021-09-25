use parsable::parsable;
use super::{FunctionDeclaration, GlobalVarDeclaration, InterfaceDeclaration, TypeDeclaration, TypedefDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    InterfaceDeclaration(InterfaceDeclaration),
    TypeDeclaration(TypeDeclaration),
    TypedefDeclaration(TypedefDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    GlobalDeclaration(GlobalVarDeclaration),
}