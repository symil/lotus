use std::rc::Rc;
use crate::{items::Identifier, utils::Link, wat};
use super::{FunctionBlueprint, ToInt, ToVasm, Type, VariableInfo, Vasm, Wat};

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
enum FunctionKind {
    Function,
    Method(Type),
    StaticMethod(Type)
}

#[derive(Debug)]
pub struct VirtualFunctionCallInfo {
    pub kind: FunctionKind,
    pub function_name: String,
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

    pub fn call_function<T : ToVasm>(function_name: &str, args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            kind: FunctionKind::Function,
            function_name: function_name.to_string(),
            args: Some(args.to_vasm()),
        })
    }

    pub fn call_method<T : ToVasm>(caller_type: &Type, function_name: &str, args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            kind: FunctionKind::Method(caller_type.clone()),
            function_name: function_name.to_string(),
            args: Some(args.to_vasm()),
        })
    }

    pub fn call_static_method<T : ToVasm>(caller_type: &Type, function_name: &str, args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            kind: FunctionKind::StaticMethod(caller_type.clone()),
            function_name: function_name.to_string(),
            args: Some(args.to_vasm()),
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
            VirtualInstruction::FunctionCall(_) => {},
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(_) => {},
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => vec![wat!["drop"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(value) => vec![Wat::const_i32(*value)],
            VirtualInstruction::FloatConstant(value) => vec![Wat::const_f32(*value)],
            VirtualInstruction::GetVariable(info) => todo!(),
            VirtualInstruction::SetVariable(_) => todo!(),
            VirtualInstruction::TeeVariable(_) => todo!(),
            VirtualInstruction::FunctionCall(info) => {
                todo!()
            },
            VirtualInstruction::Loop(info) => vec![wat!["loop", info.content.resolve()]],
            VirtualInstruction::Block(info) => {
                let mut wat = wat!["block"];

                if !info.result.is_empty() {
                    let mut result = wat!["result"];

                    for ty in &info.result {
                        if let Some(wasm_type) = ty.resolve().get_wasm_type() {
                            result.push(wasm_type);
                        }
                    }

                    wat.push(result);
                }

                wat.extend(info.content.resolve());

                vec![wat]
            },
            VirtualInstruction::Jump(info) => vec![wat!["br", info.depth]],
            VirtualInstruction::JumpIf(info) => {
                let mut jump = wat!["br_if", info.depth];

                if let Some(vasm) = &info.condition {
                    jump.extend(vasm.resolve());
                }

                vec![jump]
            },
        }
    }
}