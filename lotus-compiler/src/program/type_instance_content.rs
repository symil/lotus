use std::{collections::HashMap, hash::Hash, rc::Rc};
use indexmap::IndexMap;
use crate::{program::{FunctionInstanceParameters, TypeIndex}, utils::Link};
use super::{FunctionInstanceContent, ProgramContext, TypeBlueprint, TypeInstanceHeader, TypeInstanceParameters};

#[derive(Debug)]
pub struct TypeInstanceContent {
    // pub ancestors: Vec<Rc<TypeInstanceHeader>>
}

impl TypeInstanceContent {
    pub fn from_parameters(instance_parameters: &TypeInstanceParameters, header: Rc<TypeInstanceHeader>, context: &mut ProgramContext) -> Self {
        header.type_blueprint.with_ref(|type_unwrapped| {
            for func_ref in &type_unwrapped.dynamic_methods {
                let parameters = FunctionInstanceParameters {
                    function_blueprint: func_ref.function.clone(),
                    this_type: Some(header.clone()),
                    function_parameters: vec![],
                };

                let function_instance = context.get_function_instance(parameters);
                let index = func_ref.function.borrow().dynamic_index as usize;

                context.function_table[header.dynamic_method_table_offset + index] = Some(function_instance);
            }
        });

        // let mut ancestors = vec![];
        // let type_index = TypeIndex {
        //     current_type_instance: Some(header.clone()),
        //     current_function_parameters: vec![],
        // };

        // header.type_blueprint.with_ref(|type_unwrapped| {
        //     for ancestor in &type_unwrapped.ancestors {
        //         ancestors.push(ancestor.resolve(&type_index, context));
        //     }
        // });

        TypeInstanceContent {
            // ancestors
        }
    }
}