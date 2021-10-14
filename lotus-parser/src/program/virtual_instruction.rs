use std::rc::Rc;
use parsable::DataLocation;

use crate::{items::Identifier, program::{FunctionInstanceParameters, GeneratedItemIndex, ItemGenerator, OBJECT_HEADER_SIZE, SWAP_FLOAT_INT_WASM_FUNC_NAME, SWAP_INT_INT_WASM_FUNC_NAME, TypeInstanceParameters, VALUE_BYTE_SIZE}, utils::Link, wat};
use super::{FunctionBlueprint, ProgramContext, ToInt, ToVasm, Type, TypeBlueprint, TypeIndex, VariableInfo, VariableKind, Vasm, Wat, function_blueprint};

pub type VI = VirtualInstruction;

#[derive(Debug)]
pub enum VirtualInstruction {
    Drop,
    Raw(Wat),
    IntConstant(i32),
    FloatConstant(f32),
    Store(VirtualStashInfo),
    Load(VirtualStashInfo),
    GetVariable(VirtualGetVariableInfo),
    SetVariable(VirtualSetVariableInfo),
    TeeVariable(VirtualSetVariableInfo),
    CreateObject(VirtualCreateObjectInfo),
    GetField(VirtualGetFieldInfo),
    SetField(VirtualSetFieldInfo),
    FunctionCall(VirtualFunctionCallInfo),
    Loop(VirtualLoopInfo),
    Block(VasmInfo),
    Jump(VirtualJumpInfo),
    JumpIf(VirtualJumpIfInfo)
}

#[derive(Debug)]
pub struct VirtualStashInfo {
    pub value_type: Type,
    pub wasm_var_name: String
}

#[derive(Debug)]
pub struct VirtualGetVariableInfo {
    pub var_info: Rc<VariableInfo>,
}

#[derive(Debug)]
pub struct VirtualSetVariableInfo {
    pub var_info: Rc<VariableInfo>,
    pub value: Option<Vasm>
}

#[derive(Debug)]
pub struct VirtualCreateObjectInfo {
    pub object_type: Type,
}

#[derive(Debug)]
pub struct VirtualGetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize
}

#[derive(Debug)]
pub struct VirtualSetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize,
    pub value: Option<Vasm>
}

#[derive(Debug)]
pub struct VirtualFunctionCallInfo {
    pub caller_type: Option<Type>,
    pub function: Link<FunctionBlueprint>,
    pub parameters: Vec<Type>,
    pub args: Option<Vasm>,
}

#[derive(Debug)]
pub struct VirtualJumpInfo {
    pub depth: u32,
}

#[derive(Debug)]
pub struct VirtualJumpIfInfo {
    pub depth: u32,
    pub condition: Option<Vasm>,
}

#[derive(Debug)]
pub struct VirtualLoopInfo {
    pub content: Vasm,
}

#[derive(Debug)]
pub struct VasmInfo {
    pub result: Vec<Type>,
    pub content: Vasm,
}

impl VirtualInstruction {
    pub fn raw(value: Wat) -> Self {
        Self::Raw(value)
    }

    pub fn int<T : ToInt>(value: T) -> Self {
        Self::IntConstant(value.to_i32())
    }

    pub fn float(value: f32) -> Self {
        Self::FloatConstant(value)
    }

    pub fn store(value_type: &Type, id: u64) -> Self {
        Self::Store(VirtualStashInfo {
            value_type: value_type.clone(),
            wasm_var_name: format!("tmp_{}", id),
        })
    }

    pub fn load(value_type: &Type, id: u64) -> Self {
        Self::Load(VirtualStashInfo {
            value_type: value_type.clone(),
            wasm_var_name: format!("tmp_{}", id),
        })
    }

    pub fn get(var_info: &Rc<VariableInfo>) -> Self {
        Self::GetVariable(VirtualGetVariableInfo{
            var_info: Rc::clone(var_info),
        })
    }

    pub fn set<T : ToVasm>(var_info: &Rc<VariableInfo>, value: T) -> Self {
        Self::SetVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: Some(value.to_vasm())
        })
    }

    pub fn set_from_stack(var_info: &Rc<VariableInfo>) -> Self {
        Self::SetVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: None
        })
    }

    pub fn tee<T : ToVasm>(var_info: &Rc<VariableInfo>, value: T) -> Self {
        Self::TeeVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: Some(value.to_vasm())
        })
    }

    pub fn tee_from_stack(var_info: &Rc<VariableInfo>) -> Self {
        Self::TeeVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: None
        })
    }

    pub fn call_function<T : ToVasm>(function: Link<FunctionBlueprint>, parameters: &[Type], args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            caller_type: None,
            function,
            parameters: parameters.to_vec(),
            args: Some(args.to_vasm()),
        })
    }

    pub fn call_method<T : ToVasm>(caller_type: &Type, function: Link<FunctionBlueprint>, parameters: &[Type], args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            caller_type: Some(caller_type.clone()),
            function,
            parameters: parameters.to_vec(),
            args: Some(args.to_vasm()),
        })
    }

    pub fn call_regular_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        let function = caller_type.get_regular_method(method_name, context).unwrap().function.clone();

        Self::call_method(caller_type, function, parameters, args)
    }
    
    pub fn call_static_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        let function = caller_type.get_static_method(method_name, context).unwrap().function.clone();

        Self::call_method(caller_type, function, parameters, args)
    }

    pub fn create_object(object_type: &Type) -> Self {
        Self::CreateObject(VirtualCreateObjectInfo {
            object_type: object_type.clone(),
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
        Self::Block(VasmInfo {
            result: vec![],
            content: content.to_vasm(),
        })
    }

    pub fn typed_block<T : ToVasm>(result: Vec<Type>, content: T) -> Self {
        Self::Block(VasmInfo {
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

    pub fn collect_variables(&self, list: &mut Vec<Rc<VariableInfo>>) {
        match self {
            VirtualInstruction::Drop => {},
            VirtualInstruction::Raw(_) => {},
            VirtualInstruction::IntConstant(_) => {},
            VirtualInstruction::FloatConstant(_) => {},
            VirtualInstruction::Store(info) => {
                list.push(VariableInfo::from_wasm_name(info.wasm_var_name.to_string(), info.value_type.clone(), VariableKind::Local))
            },
            VirtualInstruction::Load(_) => {},
            VirtualInstruction::GetVariable(_) => {},
            VirtualInstruction::SetVariable(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::TeeVariable(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::CreateObject(_) => {},
            VirtualInstruction::GetField(_) => {},
            VirtualInstruction::SetField(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::FunctionCall(_) => {},
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(info) => info.condition.iter().for_each(|vasm| vasm.collect_variables(list)),
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => vec![wat!["drop"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(value) => vec![Wat::const_i32(*value)],
            VirtualInstruction::FloatConstant(value) => vec![Wat::const_f32(*value)],
            VirtualInstruction::GetVariable(info) => vec![info.var_info.get_to_stack()],
            VirtualInstruction::Store(info) => {
                vec![Wat::set_local_from_stack(&info.wasm_var_name)]
            },
            VirtualInstruction::Load(info) => {
                vec![Wat::get_local(&info.wasm_var_name)]
            },
            VirtualInstruction::SetVariable(info) | VirtualInstruction::TeeVariable(info) => {
                let mut content = vec![];

                if let Some(vasm) = &info.value {
                    content.extend(vasm.resolve(type_index, context));
                }

                if let VirtualInstruction::TeeVariable(_) = self {
                    content.push(info.var_info.tee_from_stack());
                } else {
                    content.push(info.var_info.set_from_stack());
                }

                content
            },
            VirtualInstruction::CreateObject(info) => {
                let object_type = info.object_type.resolve(type_index, context);
                let object_size = object_type.type_blueprint.borrow().fields.len() + OBJECT_HEADER_SIZE;

                vec![
                    Wat::call("mem_alloc", vec![Wat::const_i32(object_size)])
                ]
            },
            VirtualInstruction::GetField(info) => {
                let field_type = info.field_type.resolve(type_index, context);
                
                let content = vec![
                    wat!["i32.add", Wat::const_i32(info.field_offset)],
                    wat!["i32.mul", Wat::const_i32(4)],
                    wat![format!("{}.load", field_type.wasm_type.unwrap())]
                ];

                content
            },
            VirtualInstruction::SetField(info) => {
                let mut content = vec![];
                let field_type = info.field_type.resolve(type_index, context);
                let field_wasm_type = field_type.wasm_type.unwrap();

                content.push(wat!["i32.add", Wat::const_i32(info.field_offset)]);
                content.push(wat!["i32.mul", Wat::const_i32(4)]);

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

                content
            },
            VirtualInstruction::FunctionCall(info) => {
                let mut content = vec![];

                if let Some(args) = &info.args {
                    content.extend(args.resolve(type_index, context));
                }

                let this_type = info.caller_type.as_ref().and_then(|ty| Some(ty.resolve(type_index, context)));
                let function_parameters = info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect();
                let function_blueprint = info.function.with_ref(|function_unwrapped| {
                    match function_unwrapped.owner_interface.is_none() {
                        true => info.function.clone(),
                        false => this_type.as_ref().unwrap().type_blueprint.with_ref(|type_unwrapped| {
                            let is_static = function_unwrapped.is_static();
                            let index_map = match is_static {
                                true => &type_unwrapped.static_methods,
                                false => &type_unwrapped.regular_methods,
                            };

                            index_map.get(function_unwrapped.name.as_str()).unwrap().function.clone()
                        }),
                    }
                });

                let parameters = FunctionInstanceParameters {
                    function_blueprint,
                    this_type,
                    function_parameters,
                };

                let (function_instance, exists) = context.function_instances.get_header(&parameters);

                if !exists {
                    let content = parameters.generate_content(&function_instance, context);
                    
                    context.function_instances.set_content(&parameters, content);
                }

                content.extend_from_slice(&function_instance.wasm_call);

                content
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

                    wat.push(result);
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
        }
    }

    pub fn resolve_without_context(&self) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => unreachable!(),
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(_) => unreachable!(),
            VirtualInstruction::FloatConstant(_) => unreachable!(),
            VirtualInstruction::Store(_) => unreachable!(),
            VirtualInstruction::Load(_) => unreachable!(),
            VirtualInstruction::GetVariable(_) => unreachable!(),
            VirtualInstruction::SetVariable(_) => unreachable!(),
            VirtualInstruction::TeeVariable(_) => unreachable!(),
            VirtualInstruction::CreateObject(_) => unreachable!(),
            VirtualInstruction::GetField(_) => unreachable!(),
            VirtualInstruction::SetField(_) => unreachable!(),
            VirtualInstruction::FunctionCall(_) => unreachable!(),
            VirtualInstruction::Loop(_) => unreachable!(),
            VirtualInstruction::Block(_) => unreachable!(),
            VirtualInstruction::Jump(_) => unreachable!(),
            VirtualInstruction::JumpIf(_) => unreachable!(),
        }
    }
}