use parsable::parsable;

#[parsable(name="boolean")]
pub struct BooleanLiteral {
    #[parsable(regex = r"true|false")]
    pub value: String
}

impl BooleanLiteral {
    pub fn to_bool(&self) -> bool {
        match self.value.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!()
        }
    }
}