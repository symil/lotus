use std::{collections::HashMap, rc::Rc};
use indexmap::IndexMap;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{DEFAULT_FUNC_NAME, Error, NEW_FUNC_NAME, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}, vasm};
use super::{Expression, FullType, Identifier, ObjectFieldInitialization};

#[parsable]
pub struct ObjectLiteral {
    pub object_type: FullType,
    // pub field_list: Option<ObjectFieldInitializationList>
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<ObjectFieldInitialization>
}

#[parsable]
pub struct ObjectFieldInitializationList {
    #[parsable(brackets="{}", separator=",", min=1)]
    pub fields: Vec<ObjectFieldInitialization>
}

impl ObjectLiteral {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.object_type.collected_instancied_type_names(list);
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = Vasm::empty();

        if let Some(object_type) = self.object_type.process(true, context) {
            if let Type::Actual(info) = &object_type {
                let object_var = VariableInfo::new(Identifier::unique("object", self), context.int_type(), VariableKind::Local);
                let type_unwrapped = info.type_blueprint.borrow();

                if type_unwrapped.qualifier == TypeQualifier::Class {
                    let mut fields_init = HashMap::new();

                    result.extend(Vasm::undefined(vec![object_var.clone()], vec![
                        VI::call_static_method(&object_type, NEW_FUNC_NAME, &[], vec![], context),
                        VI::set_from_stack(&object_var)
                    ]));

                    for field in &self.fields {
                        if type_unwrapped.fields.get(field.name.as_str()).is_none() {
                            context.errors.add(&field.name, format!("type `{}` has no field `{}`", &object_type, &field.name));
                        }

                        if fields_init.contains_key(field.name.as_str()) {
                            context.errors.add(&field.name, format!("duplicate field initialization `{}`", &field.name));
                        }

                        if let Some(field_vasm) = field.value.process(None, context) {
                            if let Some(field_info) = type_unwrapped.fields.get(field.name.as_str()) {
                                let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);

                                if field_vasm.ty.is_assignable_to(&field_type) {
                                    fields_init.insert(field.name.as_str(), field_vasm);
                                } else {
                                    context.errors.add(&field.value, format!("expected `{}`, got `{}`", &field_type, &field_vasm.ty));
                                }
                            }
                        }
                    }

                    for (i, field_info) in type_unwrapped.fields.values().enumerate() {
                        let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);
                        let init_vasm = match fields_init.remove(&field_info.name.as_str()) {
                            Some(field_vasm) => field_vasm,
                            None => field_info.default_value.replace_type_parameters(&object_type, self.location.get_hash() + (i as u64)),
                        };

                        result.extend(vasm![
                            VI::get(&object_var),
                            VI::set_field(&field_type, field_info.offset, init_vasm)
                        ]);
                    }

                    result.extend(Vasm::new(object_type.clone(), vec![], vec![VI::get(&object_var)]));
                } else {
                    context.errors.add(&self.object_type, format!("type `{}` is not a class", &object_type));
                }
            } else {
                context.errors.add(&self.object_type, format!("cannot manually instanciate type `{}`", &object_type));
            }
        }

        match result.is_empty() {
            true => None,
            false => Some(result)
        }
    }
}