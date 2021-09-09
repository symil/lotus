use indexmap::IndexSet;
use parsable::DataLocation;
use crate::{generation::Wat, items::{EventCallbackQualifier, Identifier, Visibility}};
use super::{FunctionInstance, GlobalItem, ProgramContext, Type, WasmBlueprint};

#[derive(Debug)]
pub struct FunctionBlueprint {
    pub function_id: u64,
    pub name: Identifier,
    pub location: DataLocation,
    pub visibility: Visibility,
    pub event_callback_qualifier: Option<EventCallbackQualifier>,
    pub owner_type_id: Option<u64>,
    pub this_type: Option<Type>,
    pub payload_type: Option<Type>,
    pub conditions: Vec<(String, String)>,
    pub arguments: Vec<(Identifier, Type)>,
    pub return_type: Option<Type>,
    pub is_raw_wasm: bool,
    pub declaration: Option<Wat>,
    pub call: Vec<Wat>
}

impl FunctionBlueprint {
    pub fn to_instance(&self, generics: Vec<u64>, context: &mut ProgramContext) -> FunctionInstance {
        todo!()
    }

    pub fn is_static(&self) -> bool {
        self.this_type.is_none()
    }

    pub fn get_wasm_name(&self, context: &ProgramContext) -> String {
        let mut name = String::new();

        if let Some(type_id) = self.owner_type_id {
            name.push_str(&context.types.get_by_id(type_id).unwrap().name);
            name.push('_');
        }

        name.push_str(self.name.as_str());
        name.push('_');
        name.push_str(&self.function_id.to_string());

        name
    }
}

impl GlobalItem for FunctionBlueprint {
    fn get_id(&self) -> u64 { self.function_id }
    fn get_name(&self) -> &str { self.name.as_str() }
    fn get_location(&self) -> &DataLocation { &self.location }
    fn get_visibility(&self) -> Visibility { self.visibility }
}