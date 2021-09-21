use std::rc::Rc;

use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{AssociatedTypeInfo, FunctionBlueprint, GenericTypeInfo, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ProgramContext, RESULT_VAR_NAME, THIS_TYPE_NAME, THIS_VAR_NAME, Type, VariableInfo, VariableKind, Vasm}, utils::Link};
use super::{EventCallbackQualifier, Identifier, InterfaceAssociatedTypeDeclaration, InterfaceMethodDeclaration, InterfaceQualifier, Visibility, VisibilityWrapper};

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
            visibility: self.visibility.value.unwrap_or(Visibility::Private),
            associated_types: IndexMap::new(),
            methods: IndexMap::new(),
            static_methods: IndexMap::new(),
        };

        if context.interfaces.get_by_identifier(&self.name).is_some() {
            context.errors.add(self, format!("interface `{}` already exists", &self.name));
        }

        context.interfaces.insert(interface_blueprint);
    }

    fn process<'a, F : FnMut(Link<InterfaceBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f : F) {
        let interface_blueprint = context.interfaces.get_by_location(&self.name);

        context.current_interface = Some(interface_blueprint.clone());
        f(interface_blueprint, context);
        context.current_interface = None;
    }

    pub fn process_associated_types(&self, context: &mut ProgramContext) {
        self.process(context, |interface_wrapped, context| {
            let mut associated_types = IndexMap::new();

            for associated_type in &self.body.associated_types {
                let name = associated_type.process(context);
                let item = Rc::new(InterfaceAssociatedTypeInfo {
                    name: name.clone(),
                    required_interfaces: InterfaceList::new(vec![]),
                });

                if associated_types.insert(name.to_string(), item).is_some() {
                    context.errors.add(&associated_type.name, format!("duplicate associated type declaration `{}`", &name));
                }

                if name.as_str() == THIS_TYPE_NAME {
                    context.errors.add(&associated_type.name, format!("forbidden associated type name `{}`", THIS_TYPE_NAME));
                }
            }

            interface_wrapped.with_mut(|mut interface_unwrapped| {
                interface_unwrapped.associated_types = associated_types;
            });
        });
    }

    pub fn process_methods(&self, context: &mut ProgramContext) {
        self.process(context, |interface_wrapped, context| {
            let mut methods = IndexMap::new();

            for method in &self.body.methods {
                let (name, arguments, return_type) = method.process(context);
                let function_blueprint = FunctionBlueprint {
                    function_id: name.location.get_hash(),
                    name: name.clone(),
                    visibility: Visibility::Member,
                    event_callback_qualifier: None,
                    owner_type: None,
                    owner_interface: Some(interface_wrapped.clone()),
                    parameters: IndexMap::new(),
                    conditions: vec![],
                    this_arg: Some(VariableInfo::new(Identifier::new(THIS_VAR_NAME, self), Type::This(interface_wrapped.clone()), VariableKind::Local)),
                    payload_arg: None,
                    arguments: arguments.into_iter().map(|(name, ty)| VariableInfo::new(name, ty, VariableKind::Argument)).collect(),
                    return_value: return_type.and_then(|ty| Some(VariableInfo::new(Identifier::new(RESULT_VAR_NAME, &name), ty, VariableKind::Argument))),
                    is_dynamic: false,
                    dynamic_index: -1,
                    is_raw_wasm: false,
                    body: Vasm::empty(),
                };

                if methods.insert(name.to_string(), Link::new(function_blueprint)).is_some() {
                    context.errors.add(method, format!("duplicate method `{}`", &name));
                }
            }

            interface_wrapped.borrow_mut().methods = methods;
        });
    }
}