use parsable::parsable;
use crate::program::ProgramContext;
use super::{TopLevelBlock};

#[parsable]
pub struct LotusFile {
    pub blocks: Vec<TopLevelBlock>
}

impl LotusFile {
    pub fn process(&self, context: &mut ProgramContext) {
        let mut structs = vec![];
        let mut functions = vec![];
        let mut globals = vec![];

        for block in &self.blocks {
            match block {
                TopLevelBlock::StructDeclaration(struct_declaration) => structs.push(struct_declaration),
                TopLevelBlock::FunctionDeclaration(function_declaration) => functions.push(function_declaration),
                TopLevelBlock::GlobalDeclaration(global_declaration) => globals.push(global_declaration),
            }
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_name(index, context);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_parent(index, context);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_inheritence(index, context);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_self_fields(index, context);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_all_fields(index, context);
        }

        for (index, struct_declaration) in structs.iter().enumerate() {
            struct_declaration.process_methods_signatures(index, context);
        }

        for (index, function_declaration) in functions.iter().enumerate() {
            function_declaration.process_signature(index, context);
        }

        for (index, global_declaration) in globals.iter().enumerate() {
            global_declaration.process_declaration(index, context);
        }

        for (index, function_declaration) in functions.iter().enumerate() {
            function_declaration.process_body(index, context);
        }

        for (index, global_declaration) in globals.iter().enumerate() {
            global_declaration.process_assignment(index, context);
        }
    }
}