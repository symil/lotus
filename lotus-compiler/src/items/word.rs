use parsable::*;

#[parsable]
pub struct Word {
    #[parsable(regex = r#"[a-zA-Z_]+"#)]
    pub value: String
}