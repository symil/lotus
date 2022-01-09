use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum ParsedVarDeclarationQualifier {
    Let = "let",
    Const = "const"
}