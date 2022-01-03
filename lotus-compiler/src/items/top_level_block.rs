use parsable::parsable;
use super::{FunctionDeclaration, GlobalVarDeclaration, InterfaceDeclaration, TypeDeclaration, TypedefDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    TypedefDeclaration(TypedefDeclaration),
    InterfaceDeclaration(InterfaceDeclaration),
    TypeDeclaration(TypeDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    GlobalDeclaration(GlobalVarDeclaration),
}