use syn::{Type};

pub fn is_type(ty: &Type, name: &str) -> bool {
    get_type_name(ty) == name
}

fn get_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
        _ => todo!(),
    }
}