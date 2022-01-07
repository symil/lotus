use parsable::parsable;
use crate::program::{ProgramContext, TypedefBlueprint, Visibility};
use super::{ParsedType, Identifier, VisibilityKeywordValue, VisibilityKeyword};

#[parsable]
pub struct TypedefDeclaration {
    pub visibility: Option<VisibilityKeyword>,
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=", suffix=";")]
    pub target: ParsedType
}

impl TypedefDeclaration {
    pub fn process(&self, context: &mut ProgramContext) {
        if let Some(ty) = self.target.process(true, context) {
            context.renaming.create_area(&self.name);

            let typedef_blueprint = TypedefBlueprint {
                type_id: self.location.get_hash(),
                name: self.name.clone(),
                visibility: VisibilityKeyword::process_or(&self.visibility, Visibility::Private),
                target: ty,
            };

            context.typedefs.insert(typedef_blueprint, None);
        }
    }
}