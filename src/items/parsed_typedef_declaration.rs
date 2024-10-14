use parsable::{parsable, Token};
use crate::{program::{ProgramContext, TypedefBlueprint, Visibility, TYPE_KEYWORD}, utils::Link};
use super::{ParsedType, Identifier, ParsedVisibilityToken, ParsedVisibility, ParsedEqualToken, ParsedSemicolonToken, unwrap_item};

#[parsable]
pub struct ParsedTypedefDeclaration {
    pub visibility: Option<ParsedVisibility>,
    pub qualifier: Token<TYPE_KEYWORD>,
    pub name: Identifier,
    pub equal: ParsedEqualToken,
    pub target: Option<ParsedType>,
    pub semicolon: Option<ParsedSemicolonToken>
}

impl ParsedTypedefDeclaration {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Link<TypedefBlueprint>> {
        let mut result = None;
        let target = unwrap_item(&self.target, &self.equal, context)?;

        if let Some(ty) = target.process(true, None, context) {
            context.rename_provider.add_occurence(&self.name, &self.name);

            let typedef_blueprint = TypedefBlueprint {
                type_id: self.location.get_hash(),
                name: self.name.clone(),
                visibility: ParsedVisibility::process_or(&self.visibility, Visibility::Private),
                target: ty,
            };

            result = Some(context.typedefs.insert(typedef_blueprint, None));
        }

        unwrap_item(&self.semicolon, target, context)?;

        result
    }
}