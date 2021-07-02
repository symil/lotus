use super::type_declaration::TypeDeclaration;

item! {
    struct LotusFile {
        data_structures: Vec<TypeDeclaration>
    }
    
    entry => {
        let mut iterator = iterator!(entry);

        LotusFile {
            data_structures: parse_list!(iterator, type_declaration)
        }
    }
}