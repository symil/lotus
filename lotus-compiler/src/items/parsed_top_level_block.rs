use parsable::parsable;
use super::{ParsedFunctionDeclaration, ParsedGlobalVarDeclaration, ParsedInterfaceDeclaration, ParsedTypeDeclaration, ParsedTypedefDeclaration, ParsedVarDeclaration, ParsedTopLevelBlockInProgress, ParsedMainTypeDeclaration, ParsedRootTagDeclaration};

#[parsable]
pub enum ParsedTopLevelBlock {
    RootTagDeclaration(ParsedRootTagDeclaration),
    MainTypeDeclaration(ParsedMainTypeDeclaration),
    TypedefDeclaration(ParsedTypedefDeclaration),
    InterfaceDeclaration(ParsedInterfaceDeclaration),
    TypeDeclaration(ParsedTypeDeclaration),
    FunctionDeclaration(ParsedFunctionDeclaration),
    GlobalDeclaration(ParsedGlobalVarDeclaration),
    InProgress(ParsedTopLevelBlockInProgress)
}