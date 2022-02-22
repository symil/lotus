use parsable::parsable;

#[parsable]
pub struct ParsedVarDeclarationQualifier {
    pub token: ParsedVarDeclarationQualifierToken
}

#[parsable]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParsedVarDeclarationQualifierToken {
    Let = "let",
    Const = "const"
}