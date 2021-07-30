use crate::items::{identifier::Identifier, struct_declaration::StructQualifier};

pub fn get_builtin_method_info(name: &Identifier) -> Option<(Vec<StructQualifier>, BuiltinMethodPayload)> {
    match name.as_str() {
        "on_user_connect" | "on_user_disconnect" => Some((vec![StructQualifier::World], BuiltinMethodPayload::User)),
        "trigger" => Some((vec![StructQualifier::Event, StructQualifier::Request], BuiltinMethodPayload::None)),
        _ => None
    }
}

#[allow(dead_code)]
pub enum BuiltinMethodPayload {
    None,
    World,
    User,
    ViewInput
}