use parsable::parsable;
use super::{ParsedSuperKeyword, ParsedDot, Identifier, ParsedEqual, ParsedExpression};

#[parsable]
pub struct ParsedSuperFieldDefaultValue {
    pub super_keyword: ParsedSuperKeyword,
    pub dot: Option<ParsedDot>,
    pub field_name: Option<Identifier>,
    pub equal: Option<ParsedEqual>,
    pub expression: Option<ParsedExpression>
}

impl ParsedSuperFieldDefaultValue {
    pub fn process(&self) -> Option<(&Identifier, &ParsedExpression)> {

        None
    }
}