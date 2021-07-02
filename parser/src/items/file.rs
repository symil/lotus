use super::type_declaration::TypeDeclaration;

item! {
    struct LotusFile {
        data_structures: Vec<TypeDeclaration>
    }
    
    @entry => LotusFile {
        data_structures: parse_list!(entry, type_declaration)
    }
}