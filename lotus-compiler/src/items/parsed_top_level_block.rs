use parsable::parsable;
use super::{ParsedFunctionDeclaration, ParsedGlobalVarDeclaration, ParsedInterfaceDeclaration, ParsedTypeDeclaration, ParsedTypedefDeclaration, ParsedVarDeclaration};

#[parsable]
pub enum ParsedTopLevelBlock {
    TypedefDeclaration(ParsedTypedefDeclaration),
    InterfaceDeclaration(ParsedInterfaceDeclaration),
    TypeDeclaration(ParsedTypeDeclaration),
    FunctionDeclaration(ParsedFunctionDeclaration),
    GlobalDeclaration(ParsedGlobalVarDeclaration),
}