use std::{rc::Rc};
use parsable::parsable;
use colored::*;
use crate::program::{ActualTypeContent, AssociatedTypeContent, ProgramContext, SELF_TYPE_NAME, SELF_VAR_NAME, Type, TypeContent};
use super::{ParsedTypeArguments, Identifier, ParsedTypeSuffixToken};

#[parsable]
#[derive(Default)]
pub struct ParsedValueType {
    pub name: Identifier,
    pub arguments: ParsedTypeArguments,
    #[parsable(prefix=":", separator=":", min=1, optional=true)]
    pub associated_types: Vec<Identifier>
}

impl ParsedValueType {
    pub fn as_single_name(&self) -> Option<&Identifier> {
        match self.arguments.list.is_empty() {
            true => Some(&self.name),
            false => None
        }
    }

    pub fn collecte_instancied_type_names(&self, list: &mut Vec<String>) {
        list.push(self.name.to_string());
        // self.arguments.collecte_instancied_type_names(list);
    }

    pub fn process(&self, check_interfaces: bool, context: &mut ProgramContext) -> Option<Type> {
        let mut result = Type::undefined();
        let mut must_not_take_parameters = false;
        let mut param_count_error = false;
        let parameters = self.arguments.process(check_interfaces, context);
        let parameter_count = parameters.len();

        context.add_type_completion_area(&self.name);

        if self.name.as_str() == SELF_TYPE_NAME {
            result = context.get_this_type();
        }

        if result.is_undefined() {
            if let Some(typedef_blueprint) = context.typedefs.get_by_identifier(&self.name) {
                must_not_take_parameters = true;
                result = typedef_blueprint.borrow().target.clone();

                context.rename_provider.add_occurence(&self.name, &typedef_blueprint.borrow().name);
                context.definition_provider.set_definition(&self.name, &typedef_blueprint.borrow().name);
                context.hover_provider.set_type(&self.name, &result);
            }
        }

        if result.is_undefined() {
            if let Some(ty) = context.get_type_parameter(self.name.as_str()) {
                match ty.content() {
                    TypeContent::TypeParameter(details) => {
                        context.rename_provider.add_occurence(&self.name, &details.name);
                        context.definition_provider.set_definition(&self.name, &details.name);
                    },
                    TypeContent::FunctionParameter(details) => {
                        context.rename_provider.add_occurence(&self.name, &details.name);
                        context.definition_provider.set_definition(&self.name, &details.name);
                    },
                    TypeContent::Associated(details) => {
                        context.rename_provider.add_occurence(&self.name, &details.associated.name);
                        context.definition_provider.set_definition(&self.name, &details.associated.name);
                    },
                    _ => unreachable!()
                };

                must_not_take_parameters = true;
                result = ty;
            };
        }

        if result.is_undefined() {
            let parameter_list = parameters;

            if let Some(type_blueprint) = context.types.get_by_identifier(&self.name) {
                let parameters = &type_blueprint.borrow().parameters;

                if parameter_list.len() != parameters.len() {
                    context.errors.generic(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name.as_str().bold(), parameters.len(), parameter_list.len()));
                    param_count_error = true;
                } else {
                    for (i, (parameter, argument)) in parameters.values().zip(parameter_list.iter()).enumerate() {
                        if check_interfaces {
                            for interface_blueprint in &parameter.required_interfaces.list {
                                argument.check_match_interface(interface_blueprint, &self.arguments.list[i], context);
                            }
                        }
                    }

                    result = Type::actual(&type_blueprint, parameter_list, &self.location);
                }

                context.rename_provider.add_occurence(&self.name, &type_blueprint.borrow().name);
                context.definition_provider.set_definition(&self.name, &type_blueprint.borrow().name);
            }
        }

        if parameter_count > 0 && must_not_take_parameters{
            context.errors.generic(&self.arguments, format!("expected 0 parameter, got {}", parameter_count));
        }

        if result.is_undefined() {
            if !param_count_error {
                context.errors.generic(&self.name, format!("undefined type `{}`", &self.name.as_str().bold()));
            }
        } else {
            for name in &self.associated_types {
                if let Some(associated_type) = result.get_associated_type(name.as_str()) {
                    result = associated_type;
                } else {
                    context.errors.generic(&self.name, format!("type `{}` has no associated type `{}`", &result, name));

                    result = Type::undefined();
                    break;
                }
            }
        }

        match result.is_undefined() {
            true => None,
            false => Some(result)
        }
    }
}