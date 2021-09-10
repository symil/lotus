use indexmap::IndexMap;
use parsable::parsable;
use crate::program::{InterfaceAssociatedType, InterfaceBlueprint, InterfaceMethod, ProgramContext, THIS_TYPE_NAME};
use super::{Identifier, InterfaceAssociatedTypeDeclaration, InterfaceMethodDeclaration, InterfaceQualifier, Visibility, VisibilityWrapper};

#[parsable]
pub struct InterfaceDeclaration {
    pub visibility: VisibilityWrapper,
    pub qualifier: InterfaceQualifier,
    pub name: Identifier,
    #[parsable(brackets="{}")]
    pub body: InterfaceDeclarationBody
}

#[parsable]
pub struct InterfaceDeclarationBody {
    pub associated_types: Vec<InterfaceAssociatedTypeDeclaration>,
    pub methods: Vec<InterfaceMethodDeclaration>
}

impl InterfaceDeclaration {
    pub fn process_name(&self, context: &mut ProgramContext) {
        let mut interface_blueprint = InterfaceBlueprint {
            interface_id: self.location.get_hash(),
            name: self.name.clone(),
            location: self.location.clone(),
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            associated_types: IndexMap::new(),
            methods: IndexMap::new(),
        };

        context.interfaces.insert(interface_blueprint);
    }

    pub fn process_associated_types(&self, context: &mut ProgramContext) {
        let interface_id = self.location.get_hash();
        let mut associated_types = IndexMap::new();

        for associated_type in &self.body.associated_types {
            let name = associated_type.process(context);
            let item = InterfaceAssociatedType {
                name: name.clone(),
            };

            if associated_types.insert(name.to_string(), item).is_some() {
                context.errors.add(&associated_type.name, format!("duplicate associated type declaration `{}`", &name));
            }

            if name.as_str() == THIS_TYPE_NAME {
                context.errors.add(&associated_type.name, format!("forbidden associated type name `{}`", THIS_TYPE_NAME));
            }
        }
        
        associated_types.insert(THIS_TYPE_NAME.to_string(), InterfaceAssociatedType {
            name: Identifier::new(THIS_TYPE_NAME, &self.name)
        });

        context.interfaces.get_mut_by_id(interface_id).associated_types = associated_types;
    }

    pub fn process_methods(&self, context: &mut ProgramContext) {
        let interface_id = self.location.get_hash();
        let mut methods = IndexMap::new();

        context.current_interface = Some(interface_id);

        for method in &self.body.methods {
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