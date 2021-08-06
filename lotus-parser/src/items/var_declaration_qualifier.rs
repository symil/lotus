use parsable::parsable;

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum VarDeclarationQualifier {
    Const = "const"
}