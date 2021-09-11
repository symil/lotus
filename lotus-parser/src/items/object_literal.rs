use std::collections::HashMap;
use indexmap::IndexMap;
use parsable::parsable;
use crate::{generation::{Wat}, items::TypeQualifier, program::{Error, OBJECT_ALLOC_FUNC_NAME, ProgramContext, StructInfo, TypeOld, VariableInfo, VariableKind, IrFragment}};
use super::{Expression, Identifier, ObjectFieldInitialization};

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
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
    pub fn process(&self, context: &mut ProgramContext) -> Option<IrFragment> {
        let mut wat = vec![];
        let mut fields_init = IndexMap::new();
        let mut struct_info = StructInfo::default();

        let object_var_name = Identifier::unique("object", self).to_unique_string();
        let variables = vec![
            VariableInfo::new(object_var_name.clone(), context.int_type(), VariableKind::Local),
        ];

        if let Some(type_blueprint) = context.types.get_by_name(&self.type_name) {
            if type_blueprint.qualifier == TypeQualifier::Class {
                wat.extend(vec![
                    Wat::call(OBJECT_ALLOC_FUNC_NAME, vec![Wat::const_i32(type_blueprint.fields.len()), Wat::const_i32(type_blueprint.get_id())]),
                    Wat::set_local_from_stack(&object_var_name)
                ]);
            } else {
                context.errors.add(&self.type_name, format!("type `{}` is not a class", &self.type_name));
            }
        } else {
            context.errors.add(&self.type_name, format!("undefined type `{}`", &self.type_name));
        }

        // if let Some(field_list) = &self.field_list {
        //     for field in &field_list.fields {
        //         fields_init.push((field.name.clone(), &field.value, field.value.process(context)));
        //     }
        // }

        for field in &self.fields {
            fields_init.push((field.name.clone(), &field.value, field.value.process(context)));
        }

        if let Some(struct_annotation) = context.get_struct_by_name(&self.type_name) {
            for (field_name, field_expr, field_wasm_opt) in fields_init {
                if !struct_annotation.fields.contains_key(&field_name) {
                    errors.push(Error::located(&field_name, format!("type `{}` has no field `{}`", &self.type_name, &field_name)));
                    ok = false;
                }

                if field_initializations.contains_key(&field_name) {
                    errors.push(Error::located(&field_name, format!("field `{}`: duplicate initialization", &field_name)));
                    ok = false;
                }

                if let Some(field_wasm) = field_wasm_opt {
                    if let Some(field_info) = struct_annotation.fields.get(&field_name) {
                        if field_info.ty.is_assignable_to(&field_wasm.ty, context, &mut HashMap::new()) {
                            field_initializations.insert(field_name.clone(), field_wasm.wat);
                        } else {
                            errors.push(Error::located(field_expr, format!("field `{}`: expected type `{}`, got `{}`", &field_name, &field_info.ty, &field_wasm.ty)));
                        }
                    }
                }
            }

            for (field_name, field_info) in struct_annotation.fields.iter() {
                let field_wat = match field_initializations.remove(&field_name) {
                    Some(wat) => wat,
                    None => field_info.ty.get_default_wat(),
                };

                wat.extend(field_wat);
                wat.extend(vec![
                    Wat::get_local(&object_var_name),
                    Wat::const_i32(field_info.offset),
                    Wat::call_from_stack(field_info.ty.pointer_set_function_name())
                ]);
            }

            wat.push(Wat::get_local(&object_var_name));
        } else {
            ok = false;
        }

        context.errors.adds.extend(errors);

        match ok {
            true => Some(IrFragment::new(TypeOld::Struct(struct_info), wat, variables)),
            false => None
        }
    }
}