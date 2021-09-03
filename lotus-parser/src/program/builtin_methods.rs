use crate::items::{Identifier, TypeQualifier};

pub fn get_builtin_method_info(name: &Identifier) -> Option<(Vec<TypeQualifier>, BuiltinMethodPayload)> {
    match name.as_str() {
        "on_user_connect" | "on_user_disconnect" => Some((vec![TypeQualifier::World], BuiltinMethodPayload::User)),
        "trigger" => Some((vec![TypeQualifier::Event, TypeQualifier::Request], BuiltinMethodPayload::None)),
        _ => None
    }
}

pub enum BuiltinMethodPayload {
    None,
    World,
    User,
    ViewInput
}