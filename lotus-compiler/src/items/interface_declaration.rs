use std::rc::Rc;

use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{AssociatedTypeContent, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, ParameterTypeInfo, ProgramContext, RESULT_VAR_NAME, THIS_TYPE_NAME, THIS_VAR_NAME, Type, VariableInfo, VariableKind, Vasm}, utils::Link};
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
            regular_methods: IndexMap::new(),
            static_methods: IndexMap::new(),
        };

        if context.interfaces.get_by_identifier(&self.name).is_some() {
            context.errors.add(self, format!("interface `{}` already exists", &self.name));
        }

        context.interfaces.insert(interface_blueprint, None);
    }

    fn process<'a, F : FnMut(Link<InterfaceBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f : F) {
        let interface_blueprint = context.interfaces.get_by_location(&self.name, None);

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
            let mut regular_methods = IndexMap::new();
            let mut static_methods = IndexMap::new();

            for method in &self.body.methods {
                let (method_qualifier, name, arguments, return_type) = method.process(context);
                let method_kind = method_qualifier.to_field_kind();
                let arguments : Vec<VariableInfo> = arguments.into_iter().map(|(name, ty)| VariableInfo::from(name, ty, VariableKind::Argument)).collect();
                let mut function_blueprint = FunctionBlueprint {
                    function_id: name.location.get_hash(),
                    name: name.clone(),
                    visibility: Visibility::Member,
                    qualifier: method_qualifier,
                    event_callback_qualifier: None,
                    owner_type: None,
                    owner_interface: Some(interface_wrapped.clone()),
                    first_declared_by: None,
                    parameters: IndexMap::new(),
                    conditions: vec![],
                    this_arg: None,
                    payload_arg: None,
                    arguments,
                    return_type: return_type.unwrap_or(context.void_type()),
                    dynamic_index: -1,
                    is_raw_wasm: false,
                    body: Vasm::void(),
                };

                let index_map = match method_kind {
                    FieldKind::Static => &mut static_methods,
                    FieldKind::Regular => &mut regular_methods
                };

                let this_type = Type::This(interface_wrapped.clone());

                if !method_kind.is_static() {
                    function_blueprint.this_arg = Some(VariableInfo::from(Identifier::new(THIS_VAR_NAME, self), this_type.clone(), VariableKind::Local));
                }

                let func_ref = FuncRef {
                    function: Link::new(function_blueprint),
                    this_type: this_type,
                };

                if index_map.insert(name.to_string(), func_ref).is_some() {
                    context.errors.add(method, format!("duplicate {}method `{}`", method_kind.get_qualifier(), &name));
                }
            }

            interface_wrapped.with_mut(|mut interface_unwrapped| {
                interface_unwrapped.regular_methods = regular_methods;
                interface_unwrapped.static_methods = static_methods;
            });
        });
    }
}