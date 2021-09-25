use std::collections::HashMap;
use indexmap::IndexMap;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{DEFAULT_FUNC_NAME, Error, NEW_FUNC_NAME, ProgramContext, SET_AS_PTR_METHOD_NAME, Type, VI, VariableInfo, VariableKind, Vasm}, vasm};
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
    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = Vasm::empty();

        if let Some(object_type) = self.object_type.process(context) {
            if let Type::Actual(info) = &object_type {
                let object_var = VariableInfo::new(Identifier::unique("object", self), context.int_type(), VariableKind::Local);
                let type_unwrapped = info.type_wrapped.borrow();

                if type_unwrapped.qualifier == TypeQualifier::Class {
                    let mut fields_init = HashMap::new();

                    result.extend(Vasm::void(
                        vec![object_var.clone()],
                        vec![
                            VI::set(&object_var, vasm![VI::call_method(&object_type, object_type.get_static_method(NEW_FUNC_NAME).unwrap(), &[], vec![])])
                        ]
                    ));

                    for field in &self.fields {
                        if !type_unwrapped.fields.contains_key(field.name.as_str()) {
                            context.errors.add(&field.name, format!("type `{}` has no field `{}`", &object_type, &field.name));
                        }

                        if fields_init.contains_key(field.name.as_str()) {
                            context.errors.add(&field.name, format!("duplicate field initialization `{}`", &field.name));
                        }

                        if let Some(field_vasm) = field.value.process(context) {
                            if let Some(field_info) = type_unwrapped.fields.get(field.name.as_str()) {
                                if field_vasm.ty.is_assignable_to(&field_info.ty) {
                                    fields_init.insert(field.name.as_str(), field_vasm);
                                } else {
                                    context.errors.add(&field.value, format!("expected `{}`, got `{}`", &field_info.ty, &field_vasm.ty));
                                }
                            }
                        }
                    }

                    for field_info in type_unwrapped.fields.values() {
                        let init_vasm = match fields_init.remove(&field_info.name.as_str()) {
                            Some(field_vasm) => field_vasm,
                            None => vasm![VI::call_method(&field_info.ty, field_info.ty.get_static_method(DEFAULT_FUNC_NAME).unwrap(), &[], vec![])],
                        };

                        result.extend(vasm![
                            VI::call_method(&object_type, object_type.get_regular_method(SET_AS_PTR_METHOD_NAME).unwrap(), &[], vasm![
                                init_vasm,
                                VI::get(&object_var),
                                VI::int(field_info.offset)
                            ])
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