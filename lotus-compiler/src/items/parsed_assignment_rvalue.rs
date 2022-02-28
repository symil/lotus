use parsable::parsable;
use super::{ParsedExpression, ParsedAssignmentOperator};

#[parsable]
pub struct ParsedAssignmentRvalue {
    #[parsable(not_followed_by="=")]
    pub operator: ParsedAssignmentOperator,
    pub expression: Option<ParsedExpression>
}