use parsable::parsable;
use crate::program::ProgramContext;
use super::{FunctionDeclaration, GlobalDeclaration, TypeDeclaration, TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>,
    #[parsable(ignore)]
    pub file_name: String,
    #[parsable(ignore)]
    pub file_namespace: String
}

pub struct SortedLotusFile {
    pub structs: Vec<TypeDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub globals: Vec<GlobalDeclaration>,
    pub file_name: String,
    pub file_namespace: String
}

impl LotusFile {
    pub fn to_sorted(self) -> SortedLotusFile {
        let mut result = SortedLotusFile {
            structs: vec![],
            functions: vec![],
            globals: vec![],
            file_name: self.file_name,
            file_namespace: self.file_namespace,
        };

        for block in self.blocks {
            match block {
                TopLevelBlock::StructDeclaration(struct_declaration) => result.structs.push(struct_declaration),
                TopLevelBlock::FunctionDeclaration(function_declaration) => result.functions.push(function_declaration),
                TopLevelBlock::GlobalDeclaration(global_declaration) => result.globals.push(global_declaration),
            }
        }

        result
    }
}