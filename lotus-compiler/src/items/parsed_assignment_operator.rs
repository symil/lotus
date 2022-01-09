use parsable::parsable;

#[parsable]
pub struct ParsedAssignmentOperator {
    pub token: ParsedAssignmentOperatorToken
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum ParsedAssignmentOperatorToken {
    Equal = "=",
    PlusEqual = "+=",
    MinusEqual = "-=",
    MultEqual = "*=",
    DivEqual = "/=",
    ModEqual = "%=",
    ShlEqual = "<<=",
    ShrEqual = ">>=",
    XorEqual = "^=",
    DoubleAndEqual = "&&=",
    DoubleOrEqual = "||=",
    SingleAndEqual = "&=",
    SingleOrEqual = "|=",
}