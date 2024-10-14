use parsable::parsable;
use super::{ParsedAssignmentRvalue, ParsedIsOperation, ParsedAsOperation};

#[parsable]
pub enum ParsedOperandSuffix {
    Assignment(ParsedAssignmentRvalue),
    IsOperation(ParsedIsOperation),
    AsOperation(ParsedAsOperation)
}