use parsable::parsable;

#[parsable]
pub struct AssignmentOperator {
    pub token: AssignmentToken
}

#[parsable(impl_display=true)]
#[derive(PartialEq)]
pub enum AssignmentToken {
    Equal = "=",
    PlusEqual = "+=",
    MinusEqual = "-=",
    MultEqual = "*=",
    DivEqual = "/=",
    ModEqual = "%=",
    ShlEqual = "<<=",
    ShrEqual = ">>=",
    AndEqual = "&&=",
    OrEqual = "||=",
}