use std::collections::HashMap;

use parsable::parsable;

use crate::{generation::{OBJECT_ALLOC_FUNC_NAME, Wat}, program::{Error, ProgramContext, StructInfo, Type, Wasm}};

use super::{Expression, Identifier, ObjectFieldInitialization};

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<ObjectFieldInitialization>
}

impl ObjectLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        let mut errors = vec![];
        let mut ok = true;
        let mut wat = vec![];
        let mut fields_init = vec![];
        let mut struct_info = StructInfo::default();

        if let Some(struct_annotation) = context.get_struct_by_name(&self.type_name) {
            struct_info = struct_annotation.to_struct_info();
            todo!() // allocate the right number of values
            // wat.push(Wat::call(OBJECT_ALLOC_FUNC_NAME, vec![Wat::const_i32(struct_annotation.type_id)]));
        } else {
            context.error(&self.type_name, format!("undefined structure `{}`", &self.type_name));
            ok = false;
        }

        let mut field_initializations = HashMap::new();

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
                    errors.push(Error::located(&field_name, format!("field `{}`: duplicate declaration", &field_name)));
                    ok = false;
                }

                if let Some(field_wasm) = field_wasm_opt {
                    if let Some(field_info) = struct_annotation.fields.get(&field_name) {
                        if field_info.ty.is_assignable(&field_wasm.ty, context, &mut HashMap::new()) {
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

                wat.push(Wat::const_i32(field_info.offset));
                wat.extend(field_wat);
                wat.push(Wat::call(field_info.ty.pointer_set_function_name(), vec![]));
            }
        }

        context.errors.extend(errors);

        match ok {
            true => Some(Wasm::typed(Type::Struct(struct_info), wat)),
            false => None
        }
    }
}