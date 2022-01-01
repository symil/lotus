use crate::program::ProgramContext;
use super::LanguageServerCommandParameters;

pub fn provide_hover(parameters: &LanguageServerCommandParameters, context: &ProgramContext, lines: &mut Vec<String>) {
    if let Some((shared_identifier, location)) = parameters.get_shared_identifier_under_cursor(context) {
        if let Some(ty) = &shared_identifier.type_info {
            lines.push(format!("hover;{};{};{}", location.start, location.end, ty.to_string()));
        }
    }
}