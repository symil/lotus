use parsable::{ItemLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, BuiltinType, Type, MainType};
use super::{make_string_value_from_literal_unchecked, Identifier};

#[parsable]
pub struct ParsedMacroType {
    #[parsable(prefix="#")]
    pub token: MacroTypeToken
}

#[parsable]
pub enum MacroTypeToken {
    FieldType = "FIELD_TYPE",
    WorldType = "WORLD_TYPE",
    UserType = "USER_TYPE",
    WindowType = "WINDOW_TYPE",
    LocalDataType = "LOCAL_DATA_TYPE"
}

impl ParsedMacroType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let mut m = MacroContext::new(self, context);

        match &self.token {
            MacroTypeToken::FieldType => m.access_current_field(|field_info, context| {
                field_info.ty.clone()
            }),
            MacroTypeToken::WorldType => Some(context.get_main_type(MainType::World)),
            MacroTypeToken::UserType => Some(context.get_main_type(MainType::User)),
            MacroTypeToken::WindowType => Some(context.get_main_type(MainType::Window)),
            MacroTypeToken::LocalDataType => Some(context.get_main_type(MainType::LocalData)),
        }
    }

    pub fn process_as_name(&self, context: &mut ProgramContext) -> Option<Identifier> {
        match self.process(context) {
            Some(ty) => {
                let mut identifier = Identifier::default();

                identifier.value = ty.get_type_blueprint().borrow().name.to_string();
                identifier.location = self.location.clone();

                Some(identifier)
            },
            None => None,
        }
    }
}