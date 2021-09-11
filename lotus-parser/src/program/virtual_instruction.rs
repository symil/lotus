use std::rc::Rc;
use crate::generation::{ToInt, Wat};
use super::{Type, VariableInfo, Vasm};

pub type VI = VirtualInstruction;

#[derive(Debug)]
pub enum VirtualInstruction {
    Drop,
    Raw(Wat),
    IntConstant(i32),
    FloatConstant(f32),
    GetVariable(Rc<VariableInfo>),
    SetVariable(Rc<VariableInfo>),
    TeeVariable(Rc<VariableInfo>),
    MethodCall(VirtualMethodCallInfo),
    Loop(VirtualLoopInfo),
    Block(VasmInfo),
    Jump(VirtualJumpInfo)
}

#[derive(Debug)]
pub struct VirtualMethodCallInfo {
    pub caller_type: Type,
    pub method_name: String,
    pub args: Vec<VirtualInstruction>,
    pub is_static: bool,
}

#[derive(Debug)]
pub struct VirtualJumpInfo {
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
    pub fn int<T : ToInt>(value: T) -> Self {
        Self::IntConstant(value.to_i32())
    }

    pub fn float(value: f32) -> Self {
        Self::FloatConstant(value)
    }

    pub fn get(value: &Rc<VariableInfo>) -> Self {
        Self::GetVariable(Rc::clone(value))
    }

    pub fn set(value: &Rc<VariableInfo>) -> Self {
        Self::SetVariable(Rc::clone(value))
    }

    pub fn tee(value: &Rc<VariableInfo>) -> Self {
        Self::TeeVariable(Rc::clone(value))
    }

    pub fn method<S : ToString>(caller_type: &Type, method_name: S, args: Vec<VirtualInstruction>) -> Self {
        Self::MethodCall(VirtualMethodCallInfo {
            caller_type: caller_type.clone(),
            method_name: method_name.to_string(),
            args,
            is_static: false
        })
    }

    pub fn static_method<S : ToString>(caller_type: &Type, method_name: S, args: Vec<VirtualInstruction>) -> Self {
        Self::MethodCall(VirtualMethodCallInfo {
            caller_type: caller_type.clone(),
            method_name: method_name.to_string(),
            args,
            is_static: true
        })
    }

    pub fn loop_(content: Vasm) -> Self {
        Self::Loop(VirtualLoopInfo {
            content,
        })
    }

    pub fn block(result: Vec<Type>, content: Vasm) -> Self {
        Self::Block(VasmInfo {
            result,
            content,
        })
    }

    pub fn jump(depth: u32, condition: Option<Vasm>) -> Self {
        Self::Jump(VirtualJumpInfo {
            depth,
            condition
        })
    }

    pub fn collect_variables(&self, mut list: Vec<Rc<VariableInfo>>) -> Vec<Rc<VariableInfo>> {
        match self {
            VirtualInstruction::Drop => list,
            VirtualInstruction::Raw(_) => list,
            VirtualInstruction::IntConstant(_) => list,
            VirtualInstruction::FloatConstant(_) => list,
            VirtualInstruction::GetVariable(_) => list,
            VirtualInstruction::SetVariable(_) => list,
            VirtualInstruction::TeeVariable(_) => list,
            VirtualInstruction::MethodCall(_) => list,
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => list,
        }
    }
}