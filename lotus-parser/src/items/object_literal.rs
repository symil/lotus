use std::collections::HashMap;

use parsable::parsable;

use crate::{generation::{OBJECT_ALLOC_FUNC_NAME, Wat}, program::{ProgramContext, Type, Wasm}};

use super::{Expression, Identifier, ObjectFieldInitialization};

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<ObjectFieldInitialization>
}

impl ObjectLiteral {
    pub fn process(&self, context: &mut ProgramContext) -> Option<Wasm> {
        if let Some(struct_annotation) = context.structs.get(&self.type_name) {
            let mut ok = true;
            let mut wat = vec![
                Wat::call(OBJECT_ALLOC_FUNC_NAME, vec![Wat::const_i32(struct_annotation.type_id)])
            ];

            let mut field_initializations = HashMap::new();

            for field in &self.fields {
                if !struct_annotation.fields.contains_key(&field.name) {
                    context.error(&field.name, format!("type `{}` has no field `{}`", &self.type_name, &field.name));
                    ok = false;
                }

                if field_initializations.contains_key(&field.name) {
                    context.error(field, format!("field `{}`: duplicate declaration", &field.name));
                    ok = false;
                }

                if let Some(field_wasm) = field.value.process(context) {
                    if let Some(field_info) = struct_annotation.fields.get(&field.name) {
                        if field_info.expr_type.is_assignable(&field_wasm.ty, context, &mut HashMap::new()) {
                            field_initializations.insert(field.name.clone(), field_wasm.wat);
                        } else {
                            context.error(&field.value, format!("field `{}`: expected type `{}`, got `{}`", &field.name, &field_info.expr_type, &field_wasm.ty));
                        }
                    }
                }
            }

            for (field_name, field_info) in struct_annotation.fields.iter() {
                let field_wat = match field_initializations.remove(&field_name) {
                    Some(wat) => wat,
                    None => field_info.expr_type.get_default_wat(),
                };

                wat.push(Wat::const_i32(field_info.offset));
                wat.extend(field_wat);
                wat.push(Wat::call(field_info.expr_type.get_array_set_function_name(), vec![]));
            }

            Some(Wasm::typed(Type::Struct(self.type_name.clone()), wat))
        } else {
            None
        }
    }
}