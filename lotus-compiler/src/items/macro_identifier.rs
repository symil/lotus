use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, BuiltinType, MainType};
use super::{make_string_value_from_literal_unchecked, Identifier};

#[parsable]
pub struct MacroIdentifier {
    #[parsable(prefix="#")]
    pub value: MacroIdentifierValue
}

#[parsable]
pub enum MacroIdentifierValue {
    FieldName = "FIELD_NAME",
    WorldType = "WORLD_TYPE",
    UserType = "USER_TYPE",
    WindowType = "WINDOW_TYPE",
    LocalDataType = "LOCAL_DATA_TYPE"
}

impl MacroIdentifier {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Identifier> {
        let mut m = MacroContext::new(self, context);

        match &self.value {
            MacroIdentifierValue::FieldName => m.access_current_field(|field_info, context| {
                Identifier::new(field_info.name.as_str(), &self.location)
            }),
            MacroIdentifierValue::WorldType => get_main_type_identifier(MainType::World, self, context),
            MacroIdentifierValue::UserType => get_main_type_identifier(MainType::User, self, context),
            MacroIdentifierValue::WindowType => get_main_type_identifier(MainType::Window, self, context),
            MacroIdentifierValue::LocalDataType => get_main_type_identifier(MainType::LocalData, self, context),
        }
    }
}

pub fn get_main_type_identifier(main_type: MainType, location: &DataLocation, context: &ProgramContext) -> Option<Identifier> {
    let ty = context.get_main_type(main_type);
    let mut identifier = Identifier::default();

    identifier.value = ty.get_type_blueprint().borrow().name.to_string();
    identifier.location = location.clone();

    Some(identifier)
}