use parsable::parsable;
use super::{ParsedExpression, ParsedAssignmentOperator};

#[parsable]
pub struct ParsedAssignmentRvalue {
    pub operator: ParsedAssignmentOperator,
    pub expression: ParsedExpression
}