use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{InterfaceBlueprint, InterfaceMethod, ProgramContext};
use super::{Identifier, InterfaceMethodDeclaration, InterfaceQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct InterfaceDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: InterfaceQualifier,
    pub name: Identifier,
    #[parsable(brackets="{}")]
    pub methods: Vec<InterfaceMethodDeclaration>
}

impl InterfaceDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) {
        let mut interface_blueprint = InterfaceBlueprint {
            interface_id: self.location.get_hash(),
            name: self.name.clone(),
            location: self.location.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            methods: IndexMap::new(),
        };

        context.interfaces.insert(interface_blueprint);
    }

    pub fn process_methods(&self, context: &mut ProgramContext) {
        let interface_id = self.location.get_hash();
        let mut methods = IndexMap::new();

        context.current_interface = Some(interface_id);

        for method in &self.methods {
            let (name, arguments, return_type) = method.process(context);
            let interface_method = InterfaceMethod { name, arguments, return_type };

            if let Some(previous) = methods.insert(interface_method.name.to_string(), interface_method) {
                context.errors.add(method, format!("duplicate method `{}`", previous.name));
            }
        }

        if context.interfaces.get_by_name(&self.name).is_some() {
            context.errors.add(self, format!("interface `{}` already exists", &self.name));
        }

        context.current_interface = None;
        context.interfaces.get_mut_by_id(interface_id).methods = methods;
    }
}