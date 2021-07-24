use parsable::parsable;
use super::{function_declaration::FunctionDeclaration, statement::VarDeclaration, struct_declaration::StructDeclaration};

#[parsable]
pub enum TopLevelBlock {
    StructDeclaration(StructDeclaration),
    ConstDeclaration(VarDeclaration),
    FunctionDeclaration(FunctionDeclaration)
}

impl TopLevelBlock {
    pub fn as_struct_declaration(&self) -> &StructDeclaration {
        match self {
            Self::StructDeclaration(struct_declaration) => struct_declaration,
            _ => unreachable!()
        }
    }
}