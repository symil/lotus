use parsable::parsable;

#[parsable]
pub struct AssignmentOperator {
    pub token: AssignmentToken
}

#[parsable]
pub enum AssignmentToken {
    Equal = "="
}