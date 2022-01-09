use parsable::parsable;
use crate::program::{ProgramContext, TypedefBlueprint, Visibility};
use super::{ParsedType, Identifier, ParsedVisibilityToken, ParsedVisibility};

#[parsable]
pub struct ParsedTypedefDeclaration {
    pub visibility: Option<ParsedVisibility>,
    #[parsable(prefix="type")]
    pub name: Identifier,
    #[parsable(prefix="=", suffix=";")]
    pub target: ParsedType
}

impl ParsedTypedefDeclaration {
    pub fn process(&self, context: &mut ProgramContext) {
        if let Some(ty) = self.target.process(true, context) {
            context.rename_provider.add_occurence(&self.name, &self.name);

            let typedef_blueprint = TypedefBlueprint {
                type_id: self.location.get_hash(),
                name: self.name.clone(),
                visibility: ParsedVisibility::process_or(&self.visibility, Visibility::Private),
                target: ty,
            };

            context.typedefs.insert(typedef_blueprint, None);
        }
    }
}