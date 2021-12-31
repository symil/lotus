use std::{collections::HashSet, rc::Rc};

use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{AssociatedTypeContent, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, MethodDetails, ParameterTypeInfo, ProgramContext, ScopeKind, Signature, SELF_TYPE_NAME, SELF_VAR_NAME, Type, VariableInfo, VariableKind, Vasm}, utils::Link};
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
            context.errors.generic(self, format!("interface `{}` already exists", &self.name));
        }

        context.declare_shared_identifier(&self.name);

        context.interfaces.insert(interface_blueprint, None);
    }

    fn process<'a, F : FnMut(Link<InterfaceBlueprint>, &mut ProgramContext)>(&self, context: &mut ProgramContext, mut f : F) {
        let interface_blueprint = context.interfaces.get_by_location(&self.name, None);

        context.push_scope(ScopeKind::Interface(interface_blueprint.clone()));
        f(interface_blueprint, context);
        context.pop_scope();
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

                context.declare_shared_identifier(&name);

                if associated_types.insert(name.to_string(), item).is_some() {
                    context.errors.generic(&associated_type.name, format!("duplicate associated type declaration `{}`", &name));
                }

                if name.as_str() == SELF_TYPE_NAME {
                    context.errors.generic(&associated_type.name, format!("forbidden associated type name `{}`", SELF_TYPE_NAME));
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
                let mut function_blueprint = FunctionBlueprint {
                    function_id: name.location.get_hash(),
                    name: name.clone(),
                    visibility: Visibility::None,
                    parameters: IndexMap::new(),
                    argument_names: arguments.iter().map(|(name, ty)| name.clone()).collect(),
                    signature: Signature {
                        this_type: None,
                        argument_types: arguments.iter().map(|(name, ty)| ty.clone()).collect(),
                        return_type: return_type.unwrap_or(context.void_type()),
                    },
                    argument_variables: vec![],
                    owner_type: None,
                    owner_interface: Some(interface_wrapped.clone()),
                    is_raw_wasm: false,
                    body: Vasm::void(),
                    closure_details: None,
                    method_details: Some(MethodDetails {
                        event_callback_details: None,
                        first_declared_by: None,
                        dynamic_index: None,
                    })
                };

                let index_map = match method_kind {
                    FieldKind::Static => &mut static_methods,
                    FieldKind::Regular => &mut regular_methods
                };

                let this_type = Type::This(interface_wrapped.clone());

                if !method_kind.is_static() {
                    function_blueprint.signature.this_type = Some(this_type.clone());
                }

                let func_ref = FuncRef {
                    function: Link::new(function_blueprint),
                    this_type: this_type,
                };

                context.declare_shared_identifier(&method.name);
                context.push_scope(ScopeKind::Function(func_ref.function.clone()));
                context.pop_scope();

                if index_map.insert(name.to_string(), func_ref).is_some() {
                    context.errors.generic(method, format!("duplicate {}method `{}`", method_kind.get_qualifier(), &name));
                }
            }

            interface_wrapped.with_mut(|mut interface_unwrapped| {
                interface_unwrapped.regular_methods = regular_methods;
                interface_unwrapped.static_methods = static_methods;
            });
        });
    }
}