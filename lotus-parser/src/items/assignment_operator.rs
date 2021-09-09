use parsable::parsable;

#[parsable]
pub struct AssignmentOperatorWrapper {
    pub value: AssignmentOperator
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum AssignmentOperator {
    Equal = "=",
    PlusEqual = "+=",
    MinusEqual = "-=",
    MultEqual = "*=",
    DivEqual = "/=",
    ModEqual = "%=",
    ShlEqual = "<<=",
    ShrEqual = ">>=",
    DoubleAndEqual = "&&=",
    DoubleOrEqual = "||=",
    AndEqual = "&=",
    OrEqual = "|=",
}