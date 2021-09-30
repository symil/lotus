use std::rc::Rc;
use crate::{items::Identifier, program::{FunctionInstanceParameters, GeneratedItemIndex, ItemGenerator}, utils::Link, wat};
use super::{FunctionBlueprint, ProgramContext, ToInt, ToVasm, Type, TypeBlueprint, TypeIndex, VariableInfo, Vasm, Wat};

pub type VI = VirtualInstruction;

#[derive(Debug)]
pub enum VirtualInstruction {
    Drop,
    Raw(Wat),
    IntConstant(i32),
    FloatConstant(f32),
    GetVariable(VirtualGetVariableInfo),
    SetVariable(VirtualSetVariableInfo),
    TeeVariable(VirtualSetVariableInfo),
    GetField(VirtualGetFieldInfo),
    SetField(VirtualSetFieldInfo),
    FunctionCall(VirtualFunctionCallInfo),
    Loop(VirtualLoopInfo),
    Block(VasmInfo),
    Jump(VirtualJumpInfo),
    JumpIf(VirtualJumpIfInfo)
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
pub struct VirtualGetFieldInfo {
    pub caller_type: Type,
    pub field_name: String
}

#[derive(Debug)]
pub struct VirtualSetFieldInfo {
    pub caller_type: Type,
    pub field_name: String,
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

    pub fn get_field(caller_type: &Type, field_name: &str) -> Self {
        Self::GetField(VirtualGetFieldInfo {
            caller_type: caller_type.clone(),
            field_name: field_name.to_string(),
        })
    }

    pub fn set_field(caller_type: &Type, field_name: &str, value: Vasm) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            caller_type: caller_type.clone(),
            field_name: field_name.to_string(),
            value: Some(value),
        })
    }

    pub fn set_field_from_stack(caller_type: &Type, field_name: &str) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            caller_type: caller_type.clone(),
            field_name: field_name.to_string(),
            value: None,
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

    pub fn jump_if_from_stack<T : ToVasm>(depth: u32) -> Self {
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
            VirtualInstruction::GetVariable(_) => {},
            VirtualInstruction::SetVariable(_) => {},
            VirtualInstruction::TeeVariable(_) => {},
            VirtualInstruction::GetField(_) => {},
            VirtualInstruction::SetField(_) => {},
            VirtualInstruction::FunctionCall(_) => {},
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(_) => {},
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => vec![wat!["drop"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(value) => vec![Wat::const_i32(*value)],
            VirtualInstruction::FloatConstant(value) => vec![Wat::const_f32(*value)],
            VirtualInstruction::GetVariable(info) => vec![info.var_info.get_to_stack()],
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
            VirtualInstruction::GetField(_) => {},
            VirtualInstruction::SetField(_) => {},
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

                            index_map.get(function_unwrapped.name.as_str()).unwrap().clone()
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
            VirtualInstruction::GetVariable(_) => unreachable!(),
            VirtualInstruction::SetVariable(_) => unreachable!(),
            VirtualInstruction::TeeVariable(_) => unreachable!(),
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