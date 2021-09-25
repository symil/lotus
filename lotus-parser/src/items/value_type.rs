use std::{rc::Rc};
use parsable::parsable;
use crate::program::{ActualTypeInfo, AssociatedTypeInfo, ProgramContext, THIS_TYPE_NAME, THIS_VAR_NAME, Type};
use super::{TypeArguments, Identifier, TypeSuffix};

#[parsable]
pub struct ValueType {
    pub name: Identifier,
    pub arguments: TypeArguments,
    #[parsable(prefix="::", separator="::", min=1, optional=true)]
    pub associated_types: Vec<Identifier>
}

impl ValueType {
    pub fn as_single_name(&self) -> Option<&Identifier> {
        match self.arguments.list.is_empty() {
            true => Some(&self.name),
            false => None
        }
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Type> {
        let mut result = Type::Undefined;
        let mut associated = false;
        let mut parameter = false;
        let mut typedef = false;
        let parameters = self.arguments.process(context);
        let has_parameters = !parameters.is_empty();

        if self.name.as_str() == THIS_TYPE_NAME {
            result = context.get_this_type();
        }

        if result.is_undefined() {
            if let Some(typedef_blueprint) = context.typedefs.get_by_identifier(&self.name) {
                typedef = true;
                result = typedef_blueprint.borrow().target.clone();
            }
        }

        if result.is_undefined() {
            if let Some(current_function) = context.get_current_function() {
                if let Some(info) = current_function.borrow().parameters.get(self.name.as_str()) {
                    parameter = true;
                    result = Type::FunctionParameter(Rc::clone(info));
                }
            }
        }

        if result.is_undefined() {
            if let Some(current_interface) = context.get_current_interface() {
                if let Some(associated_type) = current_interface.borrow().associated_types.get(self.name.as_str()) {
                    associated = true;
                    result = Type::Associated(AssociatedTypeInfo {
                        root: Box::new(Type::This(current_interface.clone())),
                        associated: associated_type.clone(),
                    });
                }
            } else if let Some(current_type) = context.get_current_type() {
                if let Some(parameter_type) = current_type.borrow().parameters.get(self.name.as_str()) {
                    parameter = true;
                    result = Type::TypeParameter(Rc::clone(parameter_type));
                } else if let Some(associated_type) = current_type.borrow().associated_types.get(self.name.as_str()) {
                    associated = true;
                    result = associated_type.clone();
                }
            }
        }

        if result.is_undefined() {
            let parameter_list = parameters;

            if let Some(type_blueprint) = context.types.get_by_identifier(&self.name) {
                let parameters = &type_blueprint.borrow().parameters;

                if parameter_list.len() != parameters.len() {
                    context.errors.add(&self.name, format!("type `{}`: expected {} parameters, got {}", &self.name, parameters.len(), parameter_list.len()));
                } else {
                    for (i, (parameter, argument)) in parameters.values().zip(parameter_list.iter()).enumerate() {
                        dbg!(&parameter.required_interfaces.list);
                        for interface_blueprint in &parameter.required_interfaces.list {
                            let ok = argument.check_match_interface(interface_blueprint, &self.arguments.list[i], context);
                            dbg!(ok);
                        }
                    }

                    result = Type::Actual(ActualTypeInfo {
                        parameters: parameter_list,
                        type_wrapped: type_blueprint.clone(),
                    })
                }
            }
        }

        if has_parameters {
            if associated {
                context.errors.add(&self.arguments, format!("associated types do not take parameters"));
            } else if parameter {
                context.errors.add(&self.arguments, format!("parameter types do not take parameters"));
            } else if typedef {
                context.errors.add(&self.arguments, format!("alias types do not take parameters"));
            }
        }

        if result.is_undefined() {
            context.errors.add(&self.name, format!("undefined type `{}`", &self.name));
        } else {
            for name in &self.associated_types {
                if let Some(associated_type) = result.get_associated_type(name.as_str()) {
                    result = associated_type;
                } else {
                    context.errors.add(&self.name, format!("type `{}` has no associated type `{}`", &result, name));

                    result = Type::Undefined;
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