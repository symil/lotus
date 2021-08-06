use parsable::parsable;

use super::{ActionKeyword, Expression};

#[parsable]
pub struct Action {
    pub keyword: ActionKeyword,
    pub value: Expression
}