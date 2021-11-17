use parsable::parsable;
use super::Expression;

#[parsable]
pub struct ArgumentList {
    #[parsable(brackets="()", sep=",")]
    pub list: Vec<Expression>
}

impl ArgumentList {
    pub fn as_vec(&self) -> &Vec<Expression> {
        &self.list
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}