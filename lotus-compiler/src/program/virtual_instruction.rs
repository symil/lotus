use parsable::DataLocation;
use crate::{items::{Identifier, make_string_value_from_literal}, program::{BuiltinType, DUPLICATE_INT_WASM_FUNC_NAME, FieldKind, FunctionInstanceParameters, GeneratedItemIndex, ItemGenerator, LOAD_FLOAT_WASM_FUNC_NAME, LOAD_INT_WASM_FUNC_NAME, MEMORY_CELL_BYTE_SIZE, OBJECT_HEADER_SIZE, STORE_FLOAT_WASM_FUNC_NAME, STORE_INT_WASM_FUNC_NAME, SWAP_FLOAT_INT_WASM_FUNC_NAME, SWAP_INT_INT_WASM_FUNC_NAME, THIS_VAR_NAME, TypeInstanceHeader, TypeInstanceParameters}, utils::Link, vasm, wat};
use super::{FunctionBlueprint, FunctionCall, NamedFunctionCallDetails, ProgramContext, ToInt, ToVasm, Type, TypeBlueprint, TypeIndex, VariableInfo, VariableKind, Vasm, Wat, function_blueprint};

pub type VI = VirtualInstruction;

#[derive(Debug, Clone)]
pub enum VirtualInstruction {
    Drop(Type),
    Eqz,
    Raw(Wat),
    Return(Vasm),
    IntConstant(i32),
    FloatConstant(f32),
    TypeId(Type),
    TypeName(Type),
    InitVariable(VirtualInitVariableInfo),
    AccessVariable(VirtualVariableAccessInfo),
    GetField(VirtualGetFieldInfo),
    SetField(VirtualSetFieldInfo),
    FunctionCall(VirtualFunctionCallInfo),
    FunctionIndex(VirtualFunctionIndexInfo),
    Loop(VirtualLoopInfo),
    Block(VirtualBlockInfo),
    Jump(VirtualJumpInfo),
    JumpIf(VirtualJumpIfInfo),
    IfThenElse(IfThenElseInfo)
}

#[derive(Debug, Clone)]
pub struct VirtualInitVariableInfo {
    pub var_info: VariableInfo,
}

#[derive(Debug, Clone, Copy)]
pub enum VariableAccessKind {
    Get,
    Set,
    Tee
}

#[derive(Debug, Clone)]
pub struct VirtualVariableAccessInfo {
    pub var_info: VariableInfo,
    pub access_kind: VariableAccessKind,
    pub access_level: u32,
    pub value: Option<Vasm>,
}

#[derive(Debug, Clone)]
pub struct VirtualCreateObjectInfo {
    pub object_type: Type,
}

#[derive(Debug, Clone)]
pub struct VirtualGetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize
}

#[derive(Debug, Clone)]
pub struct VirtualSetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize,
    pub value: Option<Vasm>
}

#[derive(Debug, Clone)]
pub struct VirtualFunctionCallInfo {
    pub call: FunctionCall,
    pub function_index_var: Option<VariableInfo>,
    pub args: Vasm,
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
    pub fn raw(value: Wat) -> Self {
        Self::Raw(value)
    }

    pub fn return_value(value: Vasm) -> Self {
        Self::Return(value)
    }

    pub fn int<T : ToInt>(value: T) -> Self {
        Self::IntConstant(value.to_i32())
    }

    pub fn float(value: f32) -> Self {
        Self::FloatConstant(value)
    }

    pub fn type_id(ty: &Type) -> Self {
        Self::TypeId(ty.clone())
    }

    pub fn type_name(ty: &Type) -> Self {
        Self::TypeName(ty.clone())
    }

    pub fn init_var(var_info: &VariableInfo) -> Self {
        Self::InitVariable(VirtualInitVariableInfo {
            var_info: var_info.clone(),
        })
    }

    pub fn get_var(var_info: &VariableInfo) -> Self {
        Self::AccessVariable(VirtualVariableAccessInfo{
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Get,
            access_level: 0,
            value: None
        })
    }

    pub fn set_var<T : ToVasm>(var_info: &VariableInfo, value: T) -> Self {
        Self::AccessVariable(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Set,
            access_level: 0,
            value: Some(value.to_vasm()),
        })
    }

    pub fn set_var_from_stack(var_info: &VariableInfo) -> Self {
        Self::AccessVariable(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Set,
            access_level: 0,
            value: None,
        })
    }

    pub fn tee_var<T : ToVasm>(var_info: &VariableInfo, value: T) -> Self {
        Self::AccessVariable(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Tee,
            access_level: 0,
            value: Some(value.to_vasm()),
        })
    }

    pub fn tee_var_from_stack(var_info: &VariableInfo) -> Self {
        Self::AccessVariable(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Tee,
            access_level: 0,
            value: None,
        })
    }

    pub fn call_function<T : ToVasm>(call: FunctionCall, args: T) -> Self {
        let function_index_var = match &call {
            FunctionCall::Named(_) => None,
            FunctionCall::Anonymous(_) => Some(VariableInfo::tmp("function_index", Type::Int)),
        };

        Self::FunctionCall(VirtualFunctionCallInfo {
            call,
            function_index_var,
            args: args.to_vasm(),
        })
    }

    pub fn call_regular_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        // println!("{}: {}", caller_type, method_name);

        Self::call_function(FunctionCall::Named(NamedFunctionCallDetails {
            caller_type: Some(caller_type.clone()),
            function: caller_type.get_regular_method(method_name, context).unwrap().function.clone(),
            parameters: parameters.to_vec(),
        }), args)
    }
    
    pub fn call_static_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        // println!("{}: {}", caller_type, method_name);

        Self::call_function(FunctionCall::Named(NamedFunctionCallDetails {
            caller_type: Some(caller_type.clone()),
            function: caller_type.get_static_method(method_name, context).unwrap().function.clone(),
            parameters: parameters.to_vec(),
        }), args)
    }

    pub fn function_index(function: &Link<FunctionBlueprint>, parameters: &[Type]) -> Self {
        Self::FunctionIndex(VirtualFunctionIndexInfo {
            function: function.clone(),
            parameters: parameters.to_vec(),
        })
    }

    pub fn get_field(field_type: &Type, field_offset: usize) -> Self {
        Self::GetField(VirtualGetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
        })
    }

    pub fn set_field(field_type: &Type, field_offset: usize, value: Vasm) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
            value: Some(value)
        })
    }

    pub fn set_field_from_stack(field_type: &Type, field_offset: usize) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
            value: None
        })
    }

    pub fn loop_<T : ToVasm>(content: T) -> Self {
        Self::Loop(VirtualLoopInfo {
            content: content.to_vasm(),
        })
    }

    pub fn block<T : ToVasm>(content: T) -> Self {
        Self::Block(VirtualBlockInfo {
            result: vec![],
            content: content.to_vasm(),
        })
    }

    pub fn typed_block<T : ToVasm>(result: Vec<Type>, content: T) -> Self {
        Self::Block(VirtualBlockInfo {
            result,
            content: content.to_vasm(),
        })
    }

    pub fn jump(depth: u32) -> Self {
        Self::Jump(VirtualJumpInfo {
            depth
        })
    }

    pub fn jump_if<T : ToVasm>(depth: u32, condition: T) -> Self {
        Self::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: Some(condition.to_vasm()),
        })
    }

    pub fn jump_if_from_stack(depth: u32) -> Self {
        Self::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: None
        })
    }

    pub fn if_then_else(return_type: Option<&Type>, condition: Vasm, then_branch: Vasm, else_branch: Vasm) -> Self {
        Self::IfThenElse(IfThenElseInfo {
            return_type: return_type.cloned(),
            condition,
            then_branch,
            else_branch,
        })
    }

    pub fn collect_variables(&self, list: &mut Vec<VariableInfo>) {
        match self {
            VirtualInstruction::Drop(ty) => {},
            VirtualInstruction::Eqz => {},
            VirtualInstruction::Raw(_) => {},
            VirtualInstruction::Return(ret) => ret.collect_variables(list),
            VirtualInstruction::IntConstant(_) => {},
            VirtualInstruction::FloatConstant(_) => {},
            VirtualInstruction::TypeId(_) => {},
            VirtualInstruction::TypeName(_) => {},
            VirtualInstruction::InitVariable(info) => {},
            VirtualInstruction::AccessVariable(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::GetField(_) => {},
            VirtualInstruction::SetField(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::FunctionCall(info) => {
                info.function_index_var.iter().for_each(|var_info| list.push(var_info.clone()));
                info.args.collect_variables(list);
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

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop(ty) => match ty.resolve(type_index, context).wasm_type.is_some() {
                true => vec![wat!["drop"]],
                false => vec![],
            },
            VirtualInstruction::Eqz => vec![wat!["i32.eqz"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::Return(ret) => {
                let mut content = ret.resolve(type_index, context);
                content.push(wat!["return"]);

                content
            },
            VirtualInstruction::IntConstant(value) => vec![Wat::const_i32(*value)],
            VirtualInstruction::FloatConstant(value) => vec![Wat::const_f32(*value)],
            VirtualInstruction::TypeId(ty) => {
                let type_instance = ty.resolve(type_index, context);

                vec![Wat::const_i32(type_instance.dynamic_method_table_offset)]
            },
            VirtualInstruction::TypeName(ty) => {
                let name = ty.resolve(type_index, context).ty.get_name();
                let vasm = make_string_value_from_literal(None, &name, context).unwrap();

                vasm.resolve(type_index, context)
            },
            VirtualInstruction::InitVariable(info) => {
                info.var_info.with_ref(|var_info| {
                    match var_info.is_closure_arg {
                        true => vec![
                            Wat::call("mem_alloc", vec![Wat::const_i32(1)]),
                            Wat::set_local_from_stack(&var_info.wasm_name)
                        ],
                        false => vec![],
                    }
                })
            },
            VirtualInstruction::AccessVariable(info) => {
                let mut content = vec![];

                if let Some(vasm) = &info.value {
                    content.extend(vasm.resolve(type_index, context));
                }

                match info.var_info.ty().resolve(type_index, context).wasm_type {
                    Some(wasm_type) => info.var_info.with_ref(|var_info| {
                        match var_info.is_closure_arg {
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

                                if info.access_level == var_info.declaration_level {
                                    content.extend(vec![
                                        Wat::get_local(&var_info.wasm_name),
                                        Wat::call_from_stack(access_func_name)
                                    ]);
                                } else {
                                    content.extend(vec![
                                        Wat::get_local(THIS_VAR_NAME),
                                    ]);

                                    content.extend(vasm![
                                        VirtualInstruction::call_regular_method(&context.get_builtin_type(BuiltinType::Map, vec![context.int_type(), context.int_type()]), "get", &[], vasm![
                                            VirtualInstruction::int(var_info.name.get_u32_hash())
                                        ], context)
                                    ].resolve(type_index, context));

                                    content.push(
                                        Wat::call_from_stack(access_func_name)
                                    );
                                }
                            },
                            false => {
                                let wat = match info.access_kind {
                                    VariableAccessKind::Get => info.var_info.get_to_stack(),
                                    VariableAccessKind::Set => info.var_info.set_from_stack(),
                                    VariableAccessKind::Tee => info.var_info.tee_from_stack(),
                                };

                                content.push(wat);
                            }
                        }
                    }),
                    None => {},
                };

                content
            },
            VirtualInstruction::GetField(info) => {
                let field_type = info.field_type.resolve(type_index, context);

                match field_type.wasm_type {
                    Some(field_wasm_type) => vec![
                        wat!["i32.add", Wat::const_i32(info.field_offset)],
                        wat!["i32.mul", Wat::const_i32(4i32)],
                        wat![format!("{}.load", field_wasm_type)]
                    ],
                    None => vec![wat!["drop"]],
                }
            },
            VirtualInstruction::SetField(info) => {
                let mut content = vec![];
                let field_type = info.field_type.resolve(type_index, context);

                if let Some(field_wasm_type) = field_type.wasm_type {
                    content.push(wat!["i32.add", Wat::const_i32(info.field_offset)]);
                    content.push(wat!["i32.mul", Wat::const_i32(4i32)]);

                    if let Some(init_value) = &info.value {
                        content.extend(init_value.resolve(type_index, context));
                    } else {
                        let swap_func_name = match field_wasm_type {
                            "i32" => SWAP_INT_INT_WASM_FUNC_NAME,
                            "f32" => SWAP_FLOAT_INT_WASM_FUNC_NAME,
                            _ => unreachable!()
                        };

                        content.push(Wat::call_from_stack(swap_func_name));
                    }

                    content.push(wat![format!("{}.store", field_wasm_type)]);
                } else {
                    content.push(wat!["drop"]);
                }

                content
            },
            VirtualInstruction::FunctionCall(info) => {
                let mut content = vec![];

                match &info.call {
                    FunctionCall::Named(details) => {
                        // dbg!(details.function.borrow().name.as_str());
                        let this_type = details.caller_type.as_ref().and_then(|ty| Some(ty.resolve(type_index, context)));
                        let function_parameters = details.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect();
                        let function_blueprint = details.function.with_ref(|function_unwrapped| {
                            match &function_unwrapped.method_details {
                                Some(method_details) => match method_details.owner_interface.is_some() {
                                    true => this_type.as_ref().unwrap()
                                        .get_method(function_unwrapped.get_method_kind(), function_unwrapped.name.as_str())
                                        .unwrap(),
                                    false => details.function.clone(),
                                },
                                None => details.function.clone(),
                            }
                        });

                        let parameters = FunctionInstanceParameters {
                            function_blueprint,
                            this_type,
                            function_parameters,
                        };

                        let function_instance = context.get_function_instance(parameters);

                        content.extend(info.args.resolve(type_index, context));
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
                                    Wat::set_local_from_stack(&function_index_var.get_wasm_name())
                                ]);
                                content.extend(info.args.resolve(type_index, context));
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

                                signature_resolved.argument_types.push(Type::Int.resolve(type_index, context));
                                let closure_wasm_type_name = context.get_function_instance_wasm_type_name(&signature_resolved);

                                content.extend(vec![
                                    Wat::set_local_from_stack(&function_index_var.get_wasm_name())
                                ]);
                                content.extend(info.args.resolve(type_index, context));
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
                                            wat!["i32.add", Wat::const_i32(1), function_index_var.get_to_stack()],
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
                    this_type: None,
                    function_parameters: info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect(),
                };
                let function_instance = context.get_function_instance(parameters);
                let index : u32 = (function_instance.function_index.unwrap() as u32) | 0x80000000;
                
                vec![Wat::const_i32(index)]
            },
            VirtualInstruction::Loop(info) => vec![wat!["loop", info.content.resolve(type_index, context)]],
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
            VirtualInstruction::Jump(info) => vec![wat!["br", info.depth]],
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
            VirtualInstruction::Drop(ty) => unreachable!(),
            VirtualInstruction::Eqz => vec![wat!["i32.eqz"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::Return(_) => unreachable!(),
            VirtualInstruction::IntConstant(_) => unreachable!(),
            VirtualInstruction::FloatConstant(_) => unreachable!(),
            VirtualInstruction::TypeId(_) => unreachable!(),
            VirtualInstruction::TypeName(_) => unreachable!(),
            VirtualInstruction::InitVariable(_) => unreachable!(),
            VirtualInstruction::AccessVariable(_) => unreachable!(),
            VirtualInstruction::GetField(_) => unreachable!(),
            VirtualInstruction::SetField(_) => unreachable!(),
            VirtualInstruction::FunctionCall(_) => unreachable!(),
            VirtualInstruction::FunctionIndex(_) => unreachable!(),
            VirtualInstruction::Loop(_) => unreachable!(),
            VirtualInstruction::Block(_) => unreachable!(),
            VirtualInstruction::Jump(_) => unreachable!(),
            VirtualInstruction::JumpIf(_) => unreachable!(),
            VirtualInstruction::IfThenElse(_) => unreachable!(),
        }
    }
}