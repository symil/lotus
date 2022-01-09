use parsable::parsable;
use crate::program::ProgramContext;
use super::{ParsedFunctionDeclaration, ParsedGlobalVarDeclaration, ParsedTypeDeclaration, ParsedTopLevelBlock};

#[parsable]
#[derive(Default)]
pub struct ParsedSourceFile {
    pub blocks: Vec<ParsedTopLevelBlock>,
}