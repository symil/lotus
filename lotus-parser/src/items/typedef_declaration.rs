use parsable::parsable;
use crate::program::{ProgramContext, TypedefBlueprint};

use super::{FullType, Identifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct TypedefDeclaration {
    pub visibility: VisibilityWrapper,
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=", suffix=";")]
    pub target: FullType
}

impl TypedefDeclaration {
    pub fn process(&self, context: &mut ProgramContext) {
        if let Some(ty) = self.target.process(context) {
            let typedef_blueprint = TypedefBlueprint {
                type_id: self.location.get_hash(),
                name: self.name.clone(),
                visibility: self.visibility.value.unwrap_or(Visibility::Private),
                target: ty,
            };

            context.typedefs.insert(typedef_blueprint);
        }
    }
}