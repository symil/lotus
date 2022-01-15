use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, BuiltinType, MainType};
use super::{make_string_value_from_literal_unchecked, Identifier};

#[parsable]
pub struct ParsedMacroIdentifier {
    #[parsable(prefix="#")]
    pub token: MacroIdentifierToken
}

#[parsable]
pub enum MacroIdentifierToken {
    FieldName = "FIELD_NAME",
    WorldType = "WORLD_TYPE",
    UserType = "USER_TYPE",
    WindowType = "WINDOW_TYPE",
    LocalDataType = "LOCAL_DATA_TYPE"
}

impl ParsedMacroIdentifier {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Identifier> {
        let mut m = MacroContext::new(self, context);

        match &self.token {
            MacroIdentifierToken::FieldName => m.access_current_field(|field_info, context| {
                Identifier::new(field_info.name.as_str(), Some(&self.location))
            }),
            MacroIdentifierToken::WorldType => get_main_type_identifier(MainType::World, self, context),
            MacroIdentifierToken::UserType => get_main_type_identifier(MainType::User, self, context),
            MacroIdentifierToken::WindowType => get_main_type_identifier(MainType::Window, self, context),
            MacroIdentifierToken::LocalDataType => get_main_type_identifier(MainType::LocalData, self, context),
        }
    }
}

pub fn get_main_type_identifier(main_type: MainType, location: &ItemLocation, context: &ProgramContext) -> Option<Identifier> {
    let ty = context.get_main_type(main_type);
    let mut identifier = Identifier::default();

    identifier.value = ty.get_type_blueprint().borrow().name.to_string();
    identifier.location = location.clone();

    Some(identifier)
}