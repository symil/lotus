use crate::items::identifier::Identifier;

use super::expression_type::ExpressionType;

pub fn get_array_method_type(name: &Identifier) -> Option<ExpressionType> {
    match name.as_str() {
        _ => {}
    }

    None
}

enum Arg {
    Num,
    Item,
    Callback
}