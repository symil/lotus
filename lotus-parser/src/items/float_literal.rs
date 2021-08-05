use parsable::parsable;

#[parsable(name="float")]
pub struct FloatLiteral {
    #[parsable(regex = r"(\d+(\.\d*)?f)|nan")]
    pub value: String,
}

impl FloatLiteral {
    pub fn to_f32(&self) -> f32 {
        match self.value.as_str() {
            "nan" => f32::NAN,
            _ => self.value[0..self.value.len()-1].parse().unwrap()
        }
    }
}