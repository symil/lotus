use crate::items::identifier::Identifier;

use super::expression_type::ExpressionType;

pub fn get_builtin_field_type(type_name: &Identifier, _field_name: &Identifier) -> Option<ExpressionType> {
    match type_name.as_str() {
        _ => None
    }
}

pub fn get_array_field_type(type_name: Option<&Identifier>, field_name: &Identifier) -> Option<ExpressionType> {
    let (args, ret) = match field_name.as_str() {
        "len" => (vec![], Arg::Num),
        "filter" => (vec![Arg::BoolCallback], Arg::ArrayItem),
        "map" => (vec![Arg::MapCallback], Arg::ArrayAny),
        _ => return None
    };

    let arguments = args.into_iter().map(|arg| arg.into_expr_type(type_name)).collect();
    let return_type = ret.into_expr_type(type_name);

    Some(ExpressionType::Function(arguments, Box::new(return_type)))
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
    pub fn into_expr_type(self, type_name: Option<&Identifier>) -> ExpressionType {
        let expr_single = match type_name {
            Some(type_name) => ExpressionType::Single(type_name.clone()),
            None => ExpressionType::SingleAny,
        };

        let expr_array = match type_name {
            Some(type_name) => ExpressionType::Array(type_name.clone()),
            None => ExpressionType::ArrayAny,
        };

        match self {
            Arg::Void => ExpressionType::Void,
            Arg::Num => ExpressionType::Single(Identifier::new("num")),
            Arg::SingleItem => expr_single,
            Arg::ArrayItem => expr_array,
            Arg::ArrayAny => ExpressionType::ArrayAny,
            Arg::BoolCallback => ExpressionType::Function(vec![expr_single], Box::new(ExpressionType::Single(Identifier::new("bool")))),
            Arg::MapCallback => ExpressionType::Function(vec![expr_single], Box::new(ExpressionType::SingleAny))
        }
    }
}