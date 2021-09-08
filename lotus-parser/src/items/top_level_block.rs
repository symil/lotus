use parsable::parsable;

use super::{FunctionDeclaration, GlobalDeclaration, TypeDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    StructDeclaration(TypeDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    GlobalDeclaration(GlobalDeclaration),
}