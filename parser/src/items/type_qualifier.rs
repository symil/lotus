item! {
    enum TypeQualifier {
        Struct,
        View
    }
    
    entry => match entry.as_str() {
        "struct" => TypeQualifier::Struct,
        "view" => TypeQualifier::View,
        _ => unreachable!()
    }
}