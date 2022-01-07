use parsable::parsable;

#[parsable]
pub struct AssignmentOperator {
    pub value: AssignmentOperatorValue
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum AssignmentOperatorValue {
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