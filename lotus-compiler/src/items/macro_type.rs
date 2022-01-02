use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, MacroContext, BuiltinType, Type, MainType};
use super::{make_string_value_from_literal_unchecked, Identifier};

#[parsable]
pub struct MacroType {
    #[parsable(prefix="#")]
    pub value: MacroTypeValue
}

#[parsable]
pub enum MacroTypeValue {
    FieldType = "FIELD_TYPE",
    WorldType = "WORLD_TYPE",
    UserType = "USER_TYPE",
    WindowType = "WINDOW_TYPE",
    LocalDataType = "LOCAL_DATA_TYPE"
}

impl MacroType {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let mut m = MacroContext::new(self, context);

        match &self.value {
            MacroTypeValue::FieldType => m.access_current_field(|field_info, context| {
                field_info.ty.clone()
            }),
            MacroTypeValue::WorldType => Some(context.get_main_type(MainType::World)),
            MacroTypeValue::UserType => Some(context.get_main_type(MainType::User)),
            MacroTypeValue::WindowType => Some(context.get_main_type(MainType::Window)),
            MacroTypeValue::LocalDataType => Some(context.get_main_type(MainType::LocalData)),
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