use std::rc::Rc;
use crate::utils::Link;

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
pub struct VirtualFunctionCallInfo {
    pub function_blueprint: Link<FunctionBlueprint>,
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

    pub fn call_function<T : ToVasm>(function: Link<FunctionBlueprint>, args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            function_blueprint: function,
            args: Some(args.to_vasm()),
        })
    }

    pub fn call_function_from_stack(function: Link<FunctionBlueprint>) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            function_blueprint: function,
            args: None,
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
}