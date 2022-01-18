use parsable::*;

#[parsable]
pub struct Word {
    #[parsable(regex = r#"[a-zA-Z_][a-zA-Z0-9_]*"#)]
    pub value: String
}