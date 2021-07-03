use lotus_parsable::parsable;

use super::type_declaration::TypeDeclaration;

#[parsable]
#[derive(Debug)]
pub struct LotusFile {
    pub type_declarations: Vec<TypeDeclaration>
}