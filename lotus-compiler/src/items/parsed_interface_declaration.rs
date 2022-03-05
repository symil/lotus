use std::{collections::HashSet, rc::Rc};
use indexmap::IndexMap;
use parsable::parsable;
use crate::{program::{AssociatedTypeContent, FieldKind, FuncRef, FunctionBlueprint, InterfaceAssociatedTypeInfo, InterfaceBlueprint, InterfaceList, MethodDetails, ParameterTypeInfo, ProgramContext, ScopeKind, Signature, SELF_TYPE_NAME, SELF_VAR_NAME, Type, VariableInfo, VariableKind, Vasm, SignatureContent, Visibility}, utils::Link};
use super::{ParsedEventCallbackQualifierKeyword, Identifier, ParsedInterfaceAssociatedTypeDeclaration, ParsedInterfaceMethodDeclaration, ParsedInterfaceQualifier, ParsedVisibilityToken, ParsedVisibility};

#[parsable]
pub struct ParsedInterfaceDeclaration {
    pub visibility: Option<ParsedVisibility>,
    pub qualifier: ParsedInterfaceQualifier,
    pub name: Identifier,
    #[parsable(brackets="{}")]
    pub body: Option<ParsedInterfaceDeclarationBody>
}

#[parsable]
pub struct ParsedInterfaceDeclarationBody {
    pub associated_types: Vec<ParsedInterfaceAssociatedTypeDeclaration>,
    pub methods: Vec<ParsedInterfaceMethodDeclaration>
}

impl ParsedInterfaceDeclaration {
    fn get_associated_types(&self) -> &[ParsedInterfaceAssociatedTypeDeclaration] {
        match &self.body {
            Some(body) => &body.associated_types,
            None => &[],
        }
    }

    fn get_methods(&self) -> &[ParsedInterfaceMethodDeclaration] {
        match &self.body {
            Some(body) => &body.methods,
            None => &[],
        }
    }
    
    pub fn process_name(&self, context: &mut ProgramContext) {
        let mut interface_blueprint = InterfaceBlueprint {
            interface_id: self.location.get_hash(),
            name: self.name.clone(),
            visibility: ParsedVisibility::process_or(&self.visibility, Visibility::Private),
            associated_types: IndexMap::new(),
            regular_methods: IndexMap::new(),
            static_methods: IndexMap::new(),
        };

        if context.interfaces.get_by_identifier(&self.name).is_some() {
            context.errors.generic(self, format!("interface `{}` already exists", &self.name));
        }

        context.rename_provider.add_occurence(&self.name, &self.name);

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

            for associated_type in self.get_associated_types() {
                let name = associated_type.process(context);
                let item = Rc::new(InterfaceAssociatedTypeInfo {
                    name: name.clone(),
                    required_interfaces: InterfaceList::new(vec![]),
                });

                context.rename_provider.add_occurence(&name, &name);


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

    pub fn process_method_signatures(&self, context: &mut ProgramContext) {
        self.process(context, |interface_wrapped, context| {
            let mut regular_methods = IndexMap::new();
            let mut static_methods = IndexMap::new();

            for method in self.get_methods() {
                let function_wrapped = method.process_signature(context);
                
                function_wrapped.with_ref(|function_unwrapped| {
                    let name = function_unwrapped.name.clone();
                    let method_kind = function_unwrapped.method_details.as_ref().unwrap().qualifier.to_field_kind();
                    let index_map = match method_kind {
                        FieldKind::Static => &mut static_methods,
                        FieldKind::Regular => &mut regular_methods
                    };

                    let func_ref = FuncRef {
                        function: function_wrapped.clone(),
                        this_type: Type::this(interface_wrapped.clone()),
                    };

                    if index_map.insert(name.to_string(), func_ref).is_some() {
                        context.errors.generic(method, format!("duplicate {}method `{}`", method_kind.get_qualifier(), &name));
                    }

                    context.rename_provider.add_occurence(&method.name, &method.name);
                });

                context.push_scope(ScopeKind::Function(function_wrapped.clone()));
                context.pop_scope();
            }

            interface_wrapped.with_mut(|mut interface_unwrapped| {
                interface_unwrapped.regular_methods = regular_methods;
                interface_unwrapped.static_methods = static_methods;
            });
        });
    }

    pub fn process_method_bodies(&self, context: &mut ProgramContext) {
        self.process(context, |interface_wrapped, context| {
            for method in self.get_methods() {
                method.process_body(context);
            }
        });
    }
}