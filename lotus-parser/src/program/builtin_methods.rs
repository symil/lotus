use crate::items::identifier::Identifier;

use super::expression_type::ExpressionType;

pub enum TypeId {
    Named(Identifier),
    Anonymous(u32)
}

pub fn get_builtin_field_type(type_name: &Identifier, _field_name: &Identifier) -> Option<ExpressionType> {
    match type_name.as_str() {
        _ => None
    }
}

pub fn get_array_field_type(type_id: TypeId, field_name: &Identifier) -> Option<ExpressionType> {
    let (args, ret) = match field_name.as_str() {
        "len" => (vec![], Arg::Num),
        "get" => (vec![Arg::Num], Arg::SingleItem),
        "filter" => (vec![Arg::BoolCallback], Arg::ArrayItem),
        "map" => (vec![Arg::MapCallback], Arg::ArrayAny),
        "reverse" => (vec![], Arg::Void),
        _ => return None
    };

    let arguments = args.into_iter().map(|arg| arg.into_expr_type(&type_id)).collect();
    let return_type = ret.into_expr_type(&type_id);

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
    pub fn into_expr_type(self, type_id: &TypeId) -> ExpressionType {
        let expr_single = match type_id {
            TypeId::Named(type_name) => ExpressionType::Single(type_name.clone()),
            TypeId::Anonymous(id) => ExpressionType::SingleAny(*id),
        };

        let expr_array = match type_id {
            TypeId::Named(type_name) => ExpressionType::Array(type_name.clone()),
            TypeId::Anonymous(id) => ExpressionType::ArrayAny(*id),
        };

        let map_ret = match type_id {
            TypeId::Named(_) => ExpressionType::SingleAny(0),
            TypeId::Anonymous(id) => ExpressionType::SingleAny(id + 1),
        };

        match self {
            Arg::Void => ExpressionType::Void,
            Arg::Num => ExpressionType::Single(Identifier::new("num")),
            Arg::SingleItem => expr_single,
            Arg::ArrayItem => expr_array,
            Arg::ArrayAny => ExpressionType::ArrayAny(0),
            Arg::BoolCallback => ExpressionType::Function(vec![expr_single], Box::new(ExpressionType::Single(Identifier::new("bool")))),
            Arg::MapCallback => ExpressionType::Function(vec![expr_single], Box::new(map_ret))
        }
    }
}