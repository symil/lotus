use parsable::parsable;

#[parsable(name="string")]
pub struct StringLiteral {
    #[parsable(regex = r##""(\\.|[^"\\])*""##)]
    pub value: String
}

impl StringLiteral {
    pub fn to_actual_string(&self) -> String {
        // TODO: remove quotes and unescape the string
        self.value.clone()
    }
}

impl std::ops::Deref for StringLiteral {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for StringLiteral {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }    
}