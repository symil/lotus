use parsable::parsable;

#[parsable(name="float")]
pub struct IntegerLiteral {
    #[parsable(regex = r"(\d+)|mi")]
    pub value: String,
}

impl IntegerLiteral {
    pub fn to_i32(&self) -> i32 {
        match self.value.as_str() {
            "mi" => i32::MIN,
            _ => self.value.parse().unwrap()
        }
    }
}