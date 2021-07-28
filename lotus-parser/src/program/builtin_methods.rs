use crate::{items::identifier::Identifier};

use super::expression_type::{BuiltinType, ExpressionType};

pub fn get_builtin_field_type(builtin_type: &BuiltinType, _field_name: &Identifier) -> Option<ExpressionType> {
    match builtin_type {
        BuiltinType::Boolean => None,
        BuiltinType::Number => None,
        BuiltinType::String => None,
    }
}

pub fn get_array_field_type(item_type: &ExpressionType, field_name: &Identifier) -> Option<ExpressionType> {
    let (args, ret) = match field_name.as_str() {
        "len" => (vec![], Arg::Num),
        "get" => (vec![Arg::Num], Arg::SingleItem),
        "filter" => (vec![Arg::BoolCallback], Arg::ArrayItem),
        "map" => (vec![Arg::MapCallback], Arg::ArrayAny),
        "reverse" => (vec![], Arg::Void),
        _ => return None
    };

    let arguments = args.into_iter().map(|arg| arg.into_expr_type(item_type)).collect();
    let return_type = ret.into_expr_type(item_type);

    Some(ExpressionType::function(arguments, return_type))
}

enum Arg {
    Void,
    Num,
    SingleItem,
    ArrayItem,
    ArrayAny,
    BoolCallback,
    MapCallback
}

impl Arg {
    pub fn into_expr_type(self, item_type: &ExpressionType) -> ExpressionType {
        match self {
            Arg::Void => ExpressionType::Void,
            Arg::Num => ExpressionType::single_builtin(BuiltinType::Number),
            Arg::SingleItem => item_type.clone(),
            Arg::ArrayItem => ExpressionType::array(item_type.clone()),
            Arg::ArrayAny => ExpressionType::array(ExpressionType::Anonymous(0)),
            Arg::BoolCallback => ExpressionType::function(
                vec![item_type.clone()],
                ExpressionType::single_builtin(BuiltinType::Boolean)
            ),
            Arg::MapCallback => ExpressionType::function(
                vec![item_type.clone()],
                // picked randomly to avoid conflicts with other anonymous types
                // TODO: properly solve conflicts
                ExpressionType::Anonymous(6578436)
            )
        }
    }
}