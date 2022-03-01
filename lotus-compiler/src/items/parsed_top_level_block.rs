use parsable::parsable;
use super::{ParsedFunctionDeclaration, ParsedGlobalVarDeclaration, ParsedInterfaceDeclaration, ParsedTypeDeclaration, ParsedTypedefDeclaration, ParsedVarDeclaration, ParsedTopLevelBlockInProgress, ParsedMainTypeDeclaration};

#[parsable]
pub enum ParsedTopLevelBlock {
    MainTypeDeclaration(ParsedMainTypeDeclaration),
    TypedefDeclaration(ParsedTypedefDeclaration),
    InterfaceDeclaration(ParsedInterfaceDeclaration),
    TypeDeclaration(ParsedTypeDeclaration),
    FunctionDeclaration(ParsedFunctionDeclaration),
    GlobalDeclaration(ParsedGlobalVarDeclaration),
    InProgress(ParsedTopLevelBlockInProgress)
}