use crate::items::Identifier;

use super::{BuiltinType, ExpressionType};

pub fn get_system_variable_type(name: &Identifier) -> Option<ExpressionType> {
    match name.as_str() {
        "alloc" => Some(ExpressionType::function(vec![ExpressionType::int()], ExpressionType::int())),
        "free" => Some(ExpressionType::function(vec![ExpressionType::int()], ExpressionType::int())),
        "log_ptr" => Some(ExpressionType::function(vec![ExpressionType::int()], ExpressionType::Void)),
        "memory" => Some(ExpressionType::builtin_array(BuiltinType::Integer)),
        _ => None
    }
}