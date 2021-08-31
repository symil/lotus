use crate::{generation::{DEREF_INT_POINTER_GET_FUNC_NAME, LOG_INT_FUNC_NAME, MEMORY_RETAIN_FUNC_NAME, MEMORY_RETAIN_OBJECT_FUNC_NAME, Wat}, program::{ARRAY_BODY_ADDR_OFFSET, ARRAY_LENGTH_OFFSET, struct_annotation}, wat};
use super::{ProgramContext, StructAnnotation, Type, VariableGenerator};

#[derive(Debug)]
pub struct GeneratedMethods {
    pub retain: (String, Wat)
}

impl GeneratedMethods {
    pub fn new(struct_annotation: &StructAnnotation) -> Self {
        Self {
            retain: generate_retain_method(struct_annotation)
        }
    }
}

fn get_retain_statements(ty: &Type, value_name: &str, var_generator: &mut VariableGenerator) -> (Vec<Wat>, Vec<String>) {
    let mut locals = vec![];
    let body = match ty {
        Type::Void => unreachable!(),
        Type::System => unreachable!(),
        Type::Boolean => vec![],
        Type::Integer => vec![],
        Type::Float => vec![],
        Type::String => vec![
            Wat::call(MEMORY_RETAIN_FUNC_NAME, vec![Wat::get_local(value_name)]),
            wat!["drop"]
        ],
        Type::Null => unreachable!(),
        Type::TypeId => vec![],
        Type::Pointer(_) => todo!(),
        Type::Struct(_) => vec![
            Wat::call(MEMORY_RETAIN_OBJECT_FUNC_NAME, vec![Wat::get_local(value_name)]),
            wat!["drop"]
        ],
        Type::Array(item_type) => {
            let mut lines = vec![];
            let body_var_name = var_generator.generate("body");
            let length_var_name = var_generator.generate("length");
            let item_var_name = var_generator.generate("item");

            lines.extend(vec![
                Wat::set_local(&body_var_name, Wat::call(DEREF_INT_POINTER_GET_FUNC_NAME, vec![Wat::get_local(&value_name), Wat::const_i32(ARRAY_BODY_ADDR_OFFSET)])),
                Wat::call(MEMORY_RETAIN_FUNC_NAME, vec![Wat::get_local(&body_var_name)]),
                wat!["drop"]
            ]);

            let (item_retain_statements, item_retain_variables) = get_retain_statements(item_type, &item_var_name, var_generator);

            if !item_retain_statements.is_empty() {
                let mut while_body = vec![];

                while_body.push(Wat::set_local(&item_var_name, Wat::call(DEREF_INT_POINTER_GET_FUNC_NAME, vec![Wat::get_local(&body_var_name), Wat::const_i32(0)])));
                while_body.extend(item_retain_statements);
                while_body.push(Wat::increment_local_i32(&length_var_name, -1));
                while_body.push(Wat::increment_local_i32(&body_var_name, 1));

                lines.extend(vec![
                    Wat::set_local(&length_var_name, Wat::call(DEREF_INT_POINTER_GET_FUNC_NAME, vec![Wat::get_local(&value_name), Wat::const_i32(ARRAY_LENGTH_OFFSET)])),
                    Wat::while_loop(wat!["i32.gt_s", Wat::get_local(&length_var_name), Wat::const_i32(0)], while_body)
                ]);

                locals.extend(vec![length_var_name, item_var_name]);
                locals.extend(item_retain_variables);
            }

            locals.push(body_var_name);

            let result = wat!["block",
                Wat::call(MEMORY_RETAIN_FUNC_NAME, vec![Wat::get_local(value_name)]),
                wat!["br_if", 0, "i32.eqz"],
                lines
            ];

            vec![result]
        },
        Type::Function(_, _) => todo!(),
        Type::Any(_) => unreachable!(),
    };

    (body, locals)
}

fn generate_retain_method(struct_annotation: &StructAnnotation) -> (String, Wat) {
    const ARG_NAME : &'static str = "value";

    let mut var_generator = VariableGenerator::new();
    let mut locals_names = vec![];
    let mut lines = vec![];

    let field_value_var_name = var_generator.generate("field_value");

    for field in struct_annotation.fields.values() {
        let (field_retain_statements, field_retain_locals) = get_retain_statements(&field.ty, &field_value_var_name, &mut var_generator);

        if !field_retain_statements.is_empty() {
            lines.extend(vec![
                Wat::set_local(&field_value_var_name, Wat::call(DEREF_INT_POINTER_GET_FUNC_NAME, vec![Wat::get_local(ARG_NAME), Wat::const_i32(field.offset)])),
            ]);
            lines.extend(field_retain_statements);
            locals_names.extend(field_retain_locals);
        }
    }

    locals_names.push(field_value_var_name);

    let body = wat!["block",
        Wat::call(MEMORY_RETAIN_FUNC_NAME, vec![Wat::get_local(ARG_NAME)]),
        wat!["br_if", 0, "i32.eqz"],
        lines
    ];
    let locals = locals_names.iter().map(|name| (name.as_str(), "i32")).collect();

    let func_name = format!("{}_{}_retain", struct_annotation.get_name(), struct_annotation.get_id());
    let func_declaration = Wat::declare_function(&func_name, None, vec![(ARG_NAME, "i32")], None, locals, vec![body]);

    (func_name, func_declaration)
}