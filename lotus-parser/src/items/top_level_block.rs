use parsable::parsable;

use super::{FunctionDeclaration, StructDeclaration, VarDeclaration};

#[parsable]
pub enum TopLevelBlock {
    StructDeclaration(StructDeclaration),
    ConstDeclaration(VarDeclaration),
    FunctionDeclaration(FunctionDeclaration)
}