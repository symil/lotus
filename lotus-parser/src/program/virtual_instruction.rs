use std::rc::Rc;

use crate::generation::Wat;

use super::VariableInfo;

pub enum VirtualInstruction {
    GetVariable(Rc<VariableInfo>),
    SetVariable(Rc<VariableInfo>),
    MethodCall(VirtualMethodCallInfo)
}

pub struct VirtualMethodCallInfo {
    pub caller_type: 
}