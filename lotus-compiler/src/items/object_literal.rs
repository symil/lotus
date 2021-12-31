use std::{collections::HashMap, rc::Rc};
use colored::Colorize;
use indexmap::IndexMap;
use parsable::parsable;
use crate::{items::TypeQualifier, program::{DEFAULT_METHOD_NAME, CREATE_METHOD_NAME, ProgramContext, Type, VI, VariableInfo, VariableKind, Vasm}, vasm};
use super::{Expression, Identifier, ObjectFieldInitialization, ObjectInitializationItem, ParsedType};

#[parsable]
#[derive(Default)]
pub struct ObjectLiteral {
    pub object_type: ParsedType,
    // pub field_list: Option<ObjectFieldInitializationList>
    #[parsable(brackets="{}", separator=",")]
    pub items: Vec<ObjectInitializationItem>
}

impl ObjectLiteral {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        self.object_type.collected_instancied_type_names(list, context);
    }

    pub fn process(&self, context: &mut ProgramContext) -> Option<Vasm> {
        let mut result = Vasm::void();

        if let Some(object_type) = self.object_type.process(true, context) {
            if let Type::Actual(info) = &object_type {
                let object_var = VariableInfo::tmp("object", context.int_type());
                let type_unwrapped = info.type_blueprint.borrow();

                if type_unwrapped.is_class() {
                    let mut fields_init = HashMap::new();

                    result.extend(Vasm::new(Type::Void, vec![object_var.clone()], vec![
                        VI::call_static_method(&object_type, CREATE_METHOD_NAME, &[], vec![], context),
                        VI::set_tmp_var(&object_var)
                    ]));

                    for item in &self.items {
                        let init_info = item.process(&object_type, context);

                        for (field_name, vasm) in init_info.fields {
                            fields_init.insert(field_name, vasm);
                        }

                        if let Some(vasm) = init_info.init {
                            result.extend(vasm);
                        }
                    }

                    for (i, field_info) in type_unwrapped.fields.values().enumerate() {
                        let field_type = field_info.ty.replace_parameters(Some(&object_type), &[]);
                        let init_vasm = match fields_init.remove(field_info.name.as_str()) {
                            Some(field_vasm) => field_vasm,
                            None => field_info.default_value.clone(),
                        };

                        result.extend(vasm![
                            VI::get_tmp_var(&object_var),
                            VI::set_field(&field_type, field_info.offset, init_vasm)
                        ]);
                    }

                    result.extend(Vasm::new(object_type.clone(), vec![], vec![VI::get_tmp_var(&object_var)]));
                } else {
                    context.errors.generic(&self.object_type, format!("type `{}` is not a class", &object_type));
                }
            } else {
                context.errors.generic(&self.object_type, format!("cannot manually instanciate type `{}`", &object_type));
            }
        }

        match result.is_empty() {
            true => None,
            false => Some(result)
        }
    }
}