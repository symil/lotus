use std::collections::HashMap;

use crate::{error::{Error, ErrorList}, items::{file::LotusFile, identifier::Identifier, statement::{VarDeclarationQualifier}, top_level_block::TopLevelBlock}};

#[derive(Default)]
pub struct Context {
    pub index: HashMap<Identifier, TopLevelBlock>
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_index(&mut self, files: Vec<LotusFile>) -> Vec<Error> {
        let mut errors = ErrorList::new();

        for file in files {
            for block in file.blocks {
                let identifier = match &block {
                    TopLevelBlock::StructDeclaration(struct_declaration) => &struct_declaration.name,
                    TopLevelBlock::ConstDeclaration(const_declaration) => {
                        if const_declaration.qualifier != VarDeclarationQualifier::Const {
                            errors.add(const_declaration, "global variables must be declared with the \"const\" qualifier");
                        }

                        &const_declaration.name
                    },
                    TopLevelBlock::FunctionDeclaration(function_declaration) => &function_declaration.name,
                }.clone();

                if self.index.insert(identifier.clone(), block).is_some() {
                    errors.add(&identifier, &format!("duplicate declaration: {}", identifier));
                }
            }
        }

        errors.errors
    }
}