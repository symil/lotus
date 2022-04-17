use std::rc::Rc;
use parsable::ItemLocation;
use crate::{items::{Identifier, make_string_value_from_literal, make_string_value_from_literal_unchecked}, program::{BuiltinType, CLOSURE_TMP_VAR_NAME, CLOSURE_VARIABLES_TMP_VAR_NAME, CLOSURE_VARIABLES_VAR_NAME, DUPLICATE_INT_WASM_FUNC_NAME, FieldKind, FunctionInstanceParameters, GeneratedItemIndex, ItemGenerator, LOAD_FLOAT_WASM_FUNC_NAME, LOAD_INT_WASM_FUNC_NAME, MEMORY_CELL_BYTE_SIZE, MEM_ALLOC_FUNC_NAME, NEW_METHOD_NAME, NONE_METHOD_NAME, OBJECT_HEADER_SIZE, RETAIN_METHOD_NAME, STORE_FLOAT_WASM_FUNC_NAME, STORE_INT_WASM_FUNC_NAME, SWAP_FLOAT_INT_WASM_FUNC_NAME, SWAP_INT_INT_WASM_FUNC_NAME, SELF_VAR_NAME, TMP_VAR_NAME, TypeInstanceHeader, TypeInstanceParameters}, utils::Link, wat};
use super::{FunctionBlueprint, FunctionCall, NamedFunctionCallDetails, ProgramContext, ToInt, Type, TypeBlueprint, TypeIndex, VariableInfo, VariableKind, Vasm, Wat, function_blueprint, FunctionKind, CHECK_FIELD_ACCESS_FUNC_NAME};

#[derive(Debug, Clone)]
pub enum VirtualInstruction {
    None,
    Eqz,
    Raw(Wat),
    Drop(Type),
    Placeholder(PlaceholderDetails),
    Return(Vasm),
    IntConstant(i32),
    FloatConstant(f32),
    TypeId(Type),
    TypeName(TypeNameDetails),
    InitVariable(VirtualInitVariableInfo),
    VariableAccess(VirtualVariableAccessInfo),
    FieldAccess(VirtualAccessFieldInfo),
    FunctionCall(VirtualFunctionCallInfo),
    FunctionIndex(VirtualFunctionIndexInfo),
    Loop(VirtualLoopInfo),
    Block(VirtualBlockInfo),
    Jump(VirtualJumpInfo),
    JumpIf(VirtualJumpIfInfo),
    IfThenElse(IfThenElseInfo)
}

#[derive(Debug, Clone)]
pub struct TypeNameDetails {
    pub ty: Type,
    pub include_package: bool
}

#[derive(Debug, Clone)]
pub struct PlaceholderDetails {
    pub location: ItemLocation,
    pub vasm: Option<Rc<Vasm>>
}

#[derive(Debug, Clone)]
pub struct VirtualInitVariableInfo {
    pub var_info: VariableInfo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VariableAccessKind {
    Get,
    Set,
    Tee
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldAccessKind {
    Get,
    Set,
}


#[derive(Debug, Clone)]
pub struct VirtualVariableAccessInfo {
    pub var_info: VariableInfo,
    pub access_kind: VariableAccessKind,
    pub access_level: Option<u32>,
    pub value: Option<Vasm>,
}

#[derive(Debug, Clone)]
pub struct VirtualCreateObjectInfo {
    pub object_type: Type,
}

#[derive(Debug, Clone)]
pub struct VirtualAccessFieldInfo {
    pub acess_kind: FieldAccessKind,
    pub field_type: Type,
    pub field_offset: usize,
    pub value: Option<Vasm>,
    pub check_location: Option<ItemLocation>,
}

#[derive(Debug, Clone)]
pub struct VirtualFunctionCallInfo {
    pub call: FunctionCall,
    pub function_index_var: Option<VariableInfo>,
    pub arguments: Vec<Vasm>,
    pub check_location: Option<ItemLocation>,
}

#[derive(Debug, Clone)]
pub struct VirtualFunctionIndexInfo {
    pub function: Link<FunctionBlueprint>,
    pub parameters: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct VirtualJumpInfo {
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct VirtualJumpIfInfo {
    pub depth: u32,
    pub condition: Option<Vasm>,
}

#[derive(Debug, Clone)]
pub struct VirtualLoopInfo {
    pub content: Vasm,
}

#[derive(Debug, Clone)]
pub struct VirtualBlockInfo {
    pub result: Vec<Type>,
    pub content: Vasm,
}

#[derive(Debug, Clone)]
pub struct IfThenElseInfo {
    pub return_type: Option<Type>,
    pub condition: Vasm,
    pub then_branch: Vasm,
    pub else_branch: Vasm
}

impl VirtualInstruction {
    pub fn collect_variables(&self, list: &mut Vec<VariableInfo>) {
        match self {
            VirtualInstruction::None => {},
            VirtualInstruction::Drop(ty) => {},
            VirtualInstruction::Eqz => {},
            VirtualInstruction::Raw(_) => {},
            VirtualInstruction::Placeholder(info) => info.vasm.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::Return(ret) => ret.collect_variables(list),
            VirtualInstruction::IntConstant(_) => {},
            VirtualInstruction::FloatConstant(_) => {},
            VirtualInstruction::TypeId(_) => {},
            VirtualInstruction::TypeName(_) => {},
            VirtualInstruction::InitVariable(info) => {},
            VirtualInstruction::VariableAccess(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::FieldAccess(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::FunctionCall(info) => {
                info.function_index_var.iter().for_each(|var_info| list.push(var_info.clone()));
                info.arguments.iter().for_each(|arg| arg.collect_variables(list));
            },
            VirtualInstruction::FunctionIndex(info) => {},
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(info) => info.condition.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::IfThenElse(info) => {
                info.condition.collect_variables(list);
                info.then_branch.collect_variables(list);
                info.else_branch.collect_variables(list);
            },
        }
    }

    pub fn replace_placeholder(&mut self, location: &ItemLocation, replacement: &Rc<Vasm>) {
        match self {
            VirtualInstruction::None => {},
            VirtualInstruction::Drop(_) => {},
            VirtualInstruction::Eqz => {},
            VirtualInstruction::Raw(_) => {},
            VirtualInstruction::Placeholder(info) => {
                if &info.location == location {
                    info.vasm = Some(replacement.clone());
                }
            },
            VirtualInstruction::Return(_) => {},
            VirtualInstruction::IntConstant(_) => {},
            VirtualInstruction::FloatConstant(_) => {},
            VirtualInstruction::TypeId(_) => {},
            VirtualInstruction::TypeName(_) => {},
            VirtualInstruction::InitVariable(_) => {},
            VirtualInstruction::VariableAccess(info) => info.value.iter_mut().for_each(|vasm| vasm.replace_placeholder(location, replacement)),
            VirtualInstruction::FieldAccess(info) => info.value.iter_mut().for_each(|vasm| vasm.replace_placeholder(location, replacement)),
            VirtualInstruction::FunctionCall(info) => info.arguments.iter_mut().for_each(|arg| arg.replace_placeholder(location, replacement)),
            VirtualInstruction::FunctionIndex(_) => {},
            VirtualInstruction::Loop(info) => info.content.replace_placeholder(location, replacement),
            VirtualInstruction::Block(info) => info.content.replace_placeholder(location, replacement),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(info) => info.condition.iter_mut().for_each(|vasm| vasm.replace_placeholder(location, replacement)),
            VirtualInstruction::IfThenElse(info) => {
                info.condition.replace_placeholder(location, replacement);
                info.then_branch.replace_placeholder(location, replacement);
                info.else_branch.replace_placeholder(location, replacement);
            }
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        match self {
            VirtualInstruction::None => vec![],
            VirtualInstruction::Drop(ty) => match ty.resolve(type_index, context).wasm_type.is_some() {
                true => vec![wat!["drop"]],
                false => vec![],
            },
            VirtualInstruction::Eqz => vec![
                wat!["i32.eqz"]
            ],
            VirtualInstruction::Raw(wat) => vec![
                wat.to_owned()
            ],
            VirtualInstruction::Placeholder(info) => match &info.vasm {
                Some(vasm) => vasm.resolve(type_index, context),
                None => vec![],
            },
            VirtualInstruction::Return(ret) => {
                let mut content = ret.resolve(type_index, context);
                content.push(wat!["return"]);

                content
            },
            VirtualInstruction::IntConstant(value) => vec![
                Wat::const_i32(*value)
            ],
            VirtualInstruction::FloatConstant(value) => vec![
                Wat::const_f32(*value)
            ],
            VirtualInstruction::TypeId(ty) => {
                let type_instance = ty.resolve(type_index, context);

                vec![Wat::const_i32(type_instance.dynamic_method_table_offset)]
            },
            VirtualInstruction::TypeName(details) => {
                let resolved_type = details.ty.resolve(type_index, context);
                let type_name = resolved_type.ty.to_string();
                let name = match details.include_package {
                    true => format!("{}:{}", resolved_type.type_blueprint.borrow().name.location.get_root_directory_name(), type_name),
                    false => type_name,
                };

                let vasm = make_string_value_from_literal_unchecked(&name, context);

                vasm.resolve(type_index, context)
            },
            VirtualInstruction::InitVariable(info) => {
                // if let Some(ty) = &type_index.current_type_instance {
                //     dbg!(info.var_info.ty().to_string());
                //     dbg!(&ty.name);
                //     dbg!(info.var_info.name());
                // }
                match info.var_info.ty().resolve(type_index, context).wasm_type {
                    Some(wasm_type) => info.var_info.with_ref(|var_info| {
                        match var_info.is_closure_arg {
                            true => {
                                let mut content = vec![];

                                let (store_func_name, convert_wat) = match wasm_type {
                                    "i32" => (STORE_INT_WASM_FUNC_NAME, vec![]),
                                    "f32" => (STORE_FLOAT_WASM_FUNC_NAME, vec![wat!["f32.reinterpret_i32"]]),
                                    _ => unreachable!()
                                };

                                match var_info.kind {
                                    VariableKind::Global => unreachable!(),
                                    VariableKind::Local => {
                                        content.push(Wat::call(MEM_ALLOC_FUNC_NAME, vec![Wat::const_i32(1i32)]));
                                        content.extend(convert_wat);
                                        content.push(Wat::set_local_from_stack(&var_info.wasm_name));
                                    },
                                    VariableKind::Argument => {
                                        content.extend(vec![
                                            Wat::call(MEM_ALLOC_FUNC_NAME, vec![Wat::const_i32(1i32)]),
                                            Wat::set_global_from_stack(TMP_VAR_NAME),
                                            Wat::get_global(TMP_VAR_NAME),
                                            Wat::get_local(&var_info.wasm_name),
                                            Wat::call_from_stack(store_func_name),
                                            Wat::get_global(TMP_VAR_NAME),
                                        ]);
                                        content.extend(convert_wat);
                                        content.push(Wat::set_local_from_stack(&var_info.wasm_name));

                                    },
                                }

                                content
                            },
                            false => vec![],
                        }
                    }),
                    None => vec![]
                }
            },
            VirtualInstruction::VariableAccess(info) => {
                if info.access_kind == VariableAccessKind::Get && info.var_info.kind().is_global() {
                    
                }


                let mut content = vec![];
                let value_wat = match &info.value {
                    Some(vasm) => vasm.resolve(type_index, context),
                    None => vec![],
                };

                match info.var_info.ty().resolve(type_index, context).wasm_type {
                    Some(wasm_type) => info.var_info.with_ref(|var_info| {
                        match var_info.is_closure_arg && info.access_level.is_some() {
                            true => {
                                let access_func_name = match info.access_kind {
                                    VariableAccessKind::Get => match wasm_type {
                                        "i32" => LOAD_INT_WASM_FUNC_NAME,
                                        "f32" => LOAD_FLOAT_WASM_FUNC_NAME,
                                        _ => unreachable!()
                                    },
                                    VariableAccessKind::Set | VariableAccessKind::Tee => match wasm_type {
                                        "i32" => STORE_INT_WASM_FUNC_NAME,
                                        "f32" => STORE_FLOAT_WASM_FUNC_NAME,
                                        _ => unreachable!()
                                    },
                                };

                                if info.access_level.contains(&var_info.declaration_level) {
                                    content.push(Wat::get_local(&var_info.wasm_name));
                                    if wasm_type == "f32" {
                                        content.push(wat!["i32.reinterpret_f32"]);
                                    }
                                    content.extend(value_wat);
                                    content.push(Wat::call_from_stack(access_func_name));
                                } else {
                                    content.extend(vec![
                                        Wat::get_local(CLOSURE_VARIABLES_VAR_NAME),
                                    ]);

                                    content.extend(context.vasm()
                                        .call_regular_method(
                                            &context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), context.int_type()]), "get",
                                            &[],
                                            vec![ context.vasm().int(var_info.name.get_u32_hash()) ],
                                            context
                                        ).resolve(type_index, context)
                                    );

                                    content.extend(value_wat);
                                    content.push(Wat::call_from_stack(access_func_name));
                                }
                            },
                            false => {
                                let wat = match info.access_kind {
                                    VariableAccessKind::Get => info.var_info.get_to_stack(),
                                    VariableAccessKind::Set => info.var_info.set_from_stack(),
                                    VariableAccessKind::Tee => info.var_info.tee_from_stack(),
                                };

                                content.extend(value_wat);
                                content.push(wat);
                            }
                        }
                    }),
                    None => {},
                };

                content
            },
            VirtualInstruction::FieldAccess(info) => {
                let mut content = vec![];
                let field_type = info.field_type.resolve(type_index, context);

                if let Some(check_location) = &info.check_location {
                    add_field_access_check(check_location, &mut content, context);
                }

                if let Some(field_wasm_type) = field_type.wasm_type {
                    let wasm_instruction_name = match info.acess_kind {
                        FieldAccessKind::Get => "load",
                        FieldAccessKind::Set => "store",
                    };

                    content.extend(vec![
                        wat!["i32.add", Wat::const_i32(info.field_offset)],
                        wat!["i32.mul", Wat::const_i32(4i32)]
                    ]);

                    if let Some(init_value) = &info.value {
                        content.extend(init_value.resolve(type_index, context));
                    }

                    content.push(wat![format!("{}.{}", field_wasm_type, wasm_instruction_name)]);
                } else {
                    content.push(wat!["drop"]);
                }

                content
            },
            VirtualInstruction::FunctionCall(info) => {
                let mut content = vec![];

                match &info.call {
                    FunctionCall::Named(details) => {
                        let this_type = details.caller_type.as_ref().map(|ty| ty.resolve(type_index, context));
                        let function_parameters = details.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect();
                        let function_blueprint = match &this_type {
                            Some(ty) => details.function.with_ref(|function_unwrapped| {
                                let kind = function_unwrapped.get_method_kind();
                                let name = function_unwrapped.name.as_str();

                                match ty.get_method(kind, name) {
                                    Some(function) => function,
                                    None => {
                                        if function_unwrapped.kind != FunctionKind::DefaultValue {
                                            panic!("type `{}` has no method {:?} `{}`", &ty.name, kind, name);
                                        }
                                        details.function.clone()
                                    },
                                }
                            }),
                            None => details.function.clone(),
                        };
                        // let function_blueprint = details.function.with_ref(|function_unwrapped| {
                        //     match &function_unwrapped.owner_interface.is_some() {
                        //         true => {
                        //             // let ty = &this_type.as_ref().unwrap().ty;
                        //             // ty.print();
                        //             // dbg!(ty.get_type_blueprint().borrow().static_methods.keys().collect::<Vec<&String>>());

                        //             this_type.as_ref().unwrap()
                        //                 .get_method(function_unwrapped.get_method_kind(), function_unwrapped.name.as_str())
                        //                 .unwrap()
                        //         },
                        //         false => details.function.clone(),
                        //     }
                        // });

                        let (is_internal_function, is_empty) = function_blueprint.with_ref(|function_unwrapped| {
                            (
                                function_unwrapped.name.as_str() == RETAIN_METHOD_NAME,
                                function_unwrapped.body.is_empty()
                            )
                        });

                        if is_internal_function && is_empty {
                            return vec![];
                        }

                        // if details.function.borrow().name.is("__retain") && type_index.current_function_parameters.len() == 1 {
                        //     println!("{}", &this_type.as_ref().unwrap().ty);
                        // }

                        let parameters = FunctionInstanceParameters {
                            function_blueprint,
                            this_type,
                            function_parameters,
                        };

                        let function_instance = context.get_function_instance(parameters);

                        for arg in &info.arguments {
                            content.extend(arg.resolve(type_index, context));
                        }

                        content.extend_from_slice(&function_instance.wasm_call);
                    },
                    FunctionCall::Anonymous(details) => {
                        let resolved_signature = details.signature.resolve(type_index, context);
                        let function_index_var = info.function_index_var.as_ref().unwrap().clone();
                        let mut signature_resolved = details.signature.resolve(type_index, context);
                        let function_wasm_type_name = context.get_function_instance_wasm_type_name(&signature_resolved);

                        match &details.signature.this_type {
                            Some(ty) => {
                                content.extend(vec![
                                    Wat::call_from_stack(DUPLICATE_INT_WASM_FUNC_NAME),
                                    wat![Wat::const_i32(4i32)],
                                    wat!["i32.mul"],
                                    wat!["i32.load"],
                                    function_index_var.set_from_stack()
                                ]);
                                for arg in &info.arguments {
                                    content.extend(arg.resolve(type_index, context));
                                }
                                content.extend(vec![
                                    function_index_var.get_to_stack(),
                                    Wat::const_i32(details.function_offset),
                                    wat!["i32.add"],
                                    wat!["call_indirect", wat!["type", Wat::var_name(&function_wasm_type_name)]]
                                ]);
                            },
                            None => {
                                let mut wasm_signature = vec![];

                                for arg_type in &signature_resolved.argument_types {
                                    if let Some(wasm_type) = arg_type.wasm_type {
                                        wasm_signature.push(wat!["param", wasm_type]);
                                    }
                                }

                                if let Some(wasm_type) = signature_resolved.return_type.wasm_type {
                                    wasm_signature.push(wat!["result", wasm_type]);
                                }

                                signature_resolved.argument_types.push(context.int_type().resolve(type_index, context));
                                let closure_wasm_type_name = context.get_function_instance_wasm_type_name(&signature_resolved);

                                content.extend(vec![
                                    Wat::set_local_from_stack(&function_index_var.get_wasm_name())
                                ]);
                                for arg in &info.arguments {
                                    content.extend(arg.resolve(type_index, context));
                                }
                                content.extend(vec![
                                    function_index_var.get_to_stack(),
                                    wat!["i32.ge_u", Wat::const_i32(0x80000000u32)],
                                    wat!["if", wasm_signature,
                                        wat!["then",
                                            function_index_var.get_to_stack(),
                                            wat!["i32.and", Wat::const_i32(0x7fffffffu32)],
                                            wat!["call_indirect", wat!["type", Wat::var_name(&function_wasm_type_name)]]
                                        ],
                                        wat!["else",
                                            function_index_var.get_to_stack(),
                                            Wat::call_from_stack(LOAD_INT_WASM_FUNC_NAME),
                                            wat!["i32.add", Wat::const_i32(1i32), function_index_var.get_to_stack()],
                                            Wat::call_from_stack(LOAD_INT_WASM_FUNC_NAME),
                                            wat!["call_indirect", wat!["type", Wat::var_name(&closure_wasm_type_name)]]
                                        ]
                                    ],
                                ]);
                            },
                        }
                    },
                }

                content
            },
            VirtualInstruction::FunctionIndex(info) => {
                let parameters = FunctionInstanceParameters {
                    function_blueprint: info.function.clone(),
                    this_type: info.function.borrow().owner_type.as_ref().map(|type_wrapped| type_wrapped.borrow().self_type.resolve(type_index, context)),
                    function_parameters: info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect(),
                };
                let function_parameters = parameters.function_parameters.clone();
                let function_instance = context.get_function_instance(parameters);
                let function_index = function_instance.function_index.unwrap() as u32;

                info.function.with_ref(|function_unwrapped| {
                    match &function_unwrapped.closure_details {
                        Some(details) => {
                            let retain_function = details.retain_function.as_ref().unwrap().clone();
                            let retain_function_index = context.get_function_instance(FunctionInstanceParameters {
                                function_blueprint: retain_function.clone(),
                                this_type: None,
                                function_parameters,
                            }).function_index.unwrap();

                            let ptr_type = context.pointer_type();

                            let mut vasm = context.vasm()
                                .call_static_method(&context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), ptr_type.clone()]), NEW_METHOD_NAME, &[], vec![], context)
                                .raw(Wat::set_global_from_stack(CLOSURE_VARIABLES_TMP_VAR_NAME));

                            for var_info in &details.variables {
                                if let Some(wasm_type) = var_info.ty().resolve(type_index, context).wasm_type {
                                    let convert_vasm = match wasm_type {
                                        "i32" => context.vasm(),
                                        "f32" => context.vasm().raw(wat!["i32.reinterpret_f32"]),
                                        _ => unreachable!()
                                    };

                                    vasm = vasm
                                        .raw(Wat::get_global(CLOSURE_VARIABLES_TMP_VAR_NAME))
                                        .call_regular_method(&context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), ptr_type.clone()]), "set", &[], vec![
                                            context.vasm()
                                                .int(var_info.get_name_hash())
                                                .get_var(var_info, None)
                                                .append(convert_vasm)
                                        ], context)
                                        .drop(&context.int_type());
                                }
                            }

                            let mut wat = vasm.resolve(type_index, context);

                            wat.extend(vec![
                                Wat::call(MEM_ALLOC_FUNC_NAME, vec![
                                    Wat::const_i32(4i32)
                                ]),
                                Wat::set_global_from_stack(CLOSURE_TMP_VAR_NAME),
                                Wat::call(STORE_INT_WASM_FUNC_NAME, vec![
                                    Wat::get_global(CLOSURE_TMP_VAR_NAME),
                                    Wat::get_global(CLOSURE_VARIABLES_TMP_VAR_NAME),
                                ]),
                                Wat::call(STORE_INT_WASM_FUNC_NAME, vec![
                                    wat!["i32.add", Wat::get_global(CLOSURE_TMP_VAR_NAME), Wat::const_i32(1i32)],
                                    Wat::const_i32(function_index)
                                ]),
                                Wat::call(STORE_INT_WASM_FUNC_NAME, vec![
                                    wat!["i32.add", Wat::get_global(CLOSURE_TMP_VAR_NAME), Wat::const_i32(2i32)],
                                    Wat::const_i32(retain_function_index)
                                ]),
                                Wat::get_global(CLOSURE_TMP_VAR_NAME)
                            ]);

                            wat
                        },
                        None => {
                            let index : u32 = function_index | 0x80000000;

                            vec![
                                Wat::const_i32(index)
                            ]
                        },
                    }
                })
            },
            VirtualInstruction::Loop(info) => vec![
                wat!["loop", info.content.resolve(type_index, context)]
            ],
            VirtualInstruction::Block(info) => {
                let mut wat = wat!["block"];

                if !info.result.is_empty() {
                    let mut result = wat!["result"];

                    for ty in &info.result {
                        if let Some(wasm_type) = ty.resolve(type_index, context).wasm_type {
                            result.push(wasm_type);
                        }
                    }

                    if !result.arguments.is_empty() {
                        wat.push(result);
                    }
                }

                wat.extend(info.content.resolve(type_index, context));

                vec![wat]
            },
            VirtualInstruction::Jump(info) => vec![
                wat!["br", info.depth]
            ],
            VirtualInstruction::JumpIf(info) => {
                let mut jump = wat!["br_if", info.depth];

                if let Some(vasm) = &info.condition {
                    jump.extend(vasm.resolve(type_index, context));
                }

                vec![jump]
            },
            VirtualInstruction::IfThenElse(info) => {
                let condition_wasm = info.condition.resolve(type_index, context);
                let then_branch = info.then_branch.resolve(type_index, context);
                let else_branch = info.else_branch.resolve(type_index, context);

                let mut content = vec![];
                let mut if_block = wat!["if"];

                if let Some(return_type) = &info.return_type {
                    if let Some(wasm_type) = return_type.resolve(type_index, context).wasm_type {
                        if_block.push(wat!["result", wasm_type]);
                    }
                }

                if_block.push(wat!["then", then_branch]);
                if_block.push(wat!["else", else_branch]);

                content.extend(condition_wasm);
                content.push(if_block);

                content
            },
        }
    }

    pub fn resolve_without_context(&self) -> Vec<Wat> {
        match self {
            VirtualInstruction::None => vec![],
            VirtualInstruction::Drop(ty) => unreachable!(),
            VirtualInstruction::Eqz => vec![wat!["i32.eqz"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::Placeholder(_) => unreachable!(),
            VirtualInstruction::Return(_) => unreachable!(),
            VirtualInstruction::IntConstant(_) => unreachable!(),
            VirtualInstruction::FloatConstant(_) => unreachable!(),
            VirtualInstruction::TypeId(_) => unreachable!(),
            VirtualInstruction::TypeName(_) => unreachable!(),
            VirtualInstruction::InitVariable(_) => unreachable!(),
            VirtualInstruction::VariableAccess(_) => unreachable!(),
            VirtualInstruction::FieldAccess(_) => unreachable!(),
            VirtualInstruction::FunctionCall(_) => unreachable!(),
            VirtualInstruction::FunctionIndex(_) => unreachable!(),
            VirtualInstruction::Loop(_) => unreachable!(),
            VirtualInstruction::Block(_) => unreachable!(),
            VirtualInstruction::Jump(_) => unreachable!(),
            VirtualInstruction::JumpIf(_) => unreachable!(),
            VirtualInstruction::IfThenElse(_) => unreachable!(),
        }
    }

    pub fn resolve_as_constant(&self) -> Option<Wat> {
        match self {
            VirtualInstruction::None => None,
            VirtualInstruction::Eqz => None,
            VirtualInstruction::Raw(_) => None,
            VirtualInstruction::Drop(_) => None,
            VirtualInstruction::Placeholder(_) => None,
            VirtualInstruction::Return(_) => None,
            VirtualInstruction::IntConstant(value) => Some(Wat::const_i32(*value)),
            VirtualInstruction::FloatConstant(value) => Some(Wat::const_f32(*value)),
            VirtualInstruction::TypeId(_) => None,
            VirtualInstruction::TypeName(_) => None,
            VirtualInstruction::InitVariable(_) => None,
            VirtualInstruction::VariableAccess(_) => None,
            VirtualInstruction::FieldAccess(_) => None,
            VirtualInstruction::FunctionCall(_) => None,
            VirtualInstruction::FunctionIndex(_) => None,
            VirtualInstruction::Loop(_) => None,
            VirtualInstruction::Block(_) => None,
            VirtualInstruction::Jump(_) => None,
            VirtualInstruction::JumpIf(_) => None,
            VirtualInstruction::IfThenElse(_) => None,
        }
    }

    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Self {
        match self {
            VirtualInstruction::None => VirtualInstruction::None,
            VirtualInstruction::Eqz => VirtualInstruction::Eqz,
            VirtualInstruction::Raw(details) => VirtualInstruction::Raw(details.clone()),
            VirtualInstruction::Drop(ty) => VirtualInstruction::Drop(ty.replace_parameters(this_type, function_parameters)),
            VirtualInstruction::Placeholder(details) => VirtualInstruction::Placeholder(details.clone()),
            VirtualInstruction::Return(vasm) => VirtualInstruction::Return(vasm.replace_parameters(this_type, function_parameters)),
            VirtualInstruction::IntConstant(value) => VirtualInstruction::IntConstant(*value),
            VirtualInstruction::FloatConstant(value) => VirtualInstruction::FloatConstant(*value),
            VirtualInstruction::TypeId(ty) => VirtualInstruction::TypeId(ty.replace_parameters(this_type, function_parameters)),
            VirtualInstruction::TypeName(details) => VirtualInstruction::TypeName(TypeNameDetails {
                ty: details.ty.replace_parameters(this_type, function_parameters),
                include_package: details.include_package,
            }),
            VirtualInstruction::InitVariable(details) => VirtualInstruction::InitVariable(VirtualInitVariableInfo {
                var_info: details.var_info.clone(), // TODO
            }),
            VirtualInstruction::VariableAccess(details) => VirtualInstruction::VariableAccess(VirtualVariableAccessInfo {
                var_info: details.var_info.clone(), // TODO
                access_kind: details.access_kind,
                access_level: details.access_level,
                value: details.value.as_ref().map(|vasm| vasm.replace_parameters(this_type, function_parameters)),
            }),
            VirtualInstruction::FieldAccess(details) => VirtualInstruction::FieldAccess(VirtualAccessFieldInfo {
                acess_kind: details.acess_kind,
                field_type: details.field_type.replace_parameters(this_type, function_parameters),
                field_offset: details.field_offset,
                value: details.value.as_ref().map(|vasm| vasm.replace_parameters(this_type, function_parameters)),
                check_location: details.check_location.clone(),
            }),
            VirtualInstruction::FunctionCall(details) => VirtualInstruction::FunctionCall(VirtualFunctionCallInfo {
                call: details.call.replace_parameters(this_type, function_parameters),
                function_index_var: details.function_index_var.clone(), // TODO
                arguments: details.arguments.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
                check_location: details.check_location.clone(),
            }),
            VirtualInstruction::FunctionIndex(details) => VirtualInstruction::FunctionIndex(VirtualFunctionIndexInfo {
                function: details.function.clone(),
                parameters: details.parameters.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            }),
            VirtualInstruction::Loop(details) => VirtualInstruction::Loop(VirtualLoopInfo {
                content: details.content.replace_parameters(this_type, function_parameters),
            }),
            VirtualInstruction::Block(details) => VirtualInstruction::Block(VirtualBlockInfo {
                result: details.result.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
                content: details.content.replace_parameters(this_type, function_parameters),
            }),
            VirtualInstruction::Jump(details) => VirtualInstruction::Jump(VirtualJumpInfo {
                depth: details.depth,
            }),
            VirtualInstruction::JumpIf(details) => VirtualInstruction::JumpIf(VirtualJumpIfInfo {
                depth: details.depth,
                condition: details.condition.as_ref().map(|ty| ty.replace_parameters(this_type, function_parameters)),
            }),
            VirtualInstruction::IfThenElse(details) => VirtualInstruction::IfThenElse(IfThenElseInfo {
                return_type: details.return_type.as_ref().map(|ty| ty.replace_parameters(this_type, function_parameters)),
                condition: details.condition.replace_parameters(this_type, function_parameters),
                then_branch: details.then_branch.replace_parameters(this_type, function_parameters),
                else_branch: details.else_branch.replace_parameters(this_type, function_parameters),
            }),
        }
    }
}

fn add_field_access_check(check_location: &ItemLocation, content: &mut Vec<Wat>, context: &mut ProgramContext) {
    let field_name_var = context.string_literals.add(&check_location.as_str());
    let file_name_var = context.string_literals.add(&check_location.file.path[check_location.file.package_root_path.len()+1..]);
    let (line, column) = context.line_col_index.lookup(check_location);

    content.extend(vec![
        Wat::get_global(&field_name_var.wasm_name()),
        Wat::get_global(&file_name_var.wasm_name()),
        Wat::const_i32(line),
        Wat::const_i32(column),
        Wat::call_from_stack(CHECK_FIELD_ACCESS_FUNC_NAME)
    ]);
}