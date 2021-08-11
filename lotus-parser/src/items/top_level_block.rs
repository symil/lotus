use parsable::parsable;

use super::{FunctionDeclaration, GlobalDeclaration, StructDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    StructDeclaration(StructDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    GlobalDeclaration(GlobalDeclaration),
}