use crate::utils::Link;
use super::{FunctionBlueprint, Signature, Type};

#[derive(Debug, Clone)]
pub enum FunctionCall {
    Named(NamedFunctionCallDetails),
    Anonymous(AnonymousFunctionCallDetails),
}

#[derive(Debug, Clone)]
pub struct NamedFunctionCallDetails {
    pub caller_type: Option<Type>,
    pub function: Link<FunctionBlueprint>,
    pub parameters: Vec<Type>
}

#[derive(Debug, Clone)]
pub struct AnonymousFunctionCallDetails {
    pub signature: Signature,
    pub function_offset: usize
}

impl FunctionCall {
    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Self {
        match self {
            FunctionCall::Named(details) => FunctionCall::Named(NamedFunctionCallDetails {
                caller_type: details.caller_type.as_ref().map(|ty| ty.replace_parameters(this_type, function_parameters)),
                function: details.function.clone(),
                parameters: details.parameters.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            }),
            FunctionCall::Anonymous(details) => FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                signature: details.signature.replace_parameters(this_type, function_parameters),
                function_offset: details.function_offset,
            }),
        }
    }

    pub fn get_parameters(&self) -> &[Type] {
        match self {
            FunctionCall::Named(details) => details.parameters.as_slice(),
            FunctionCall::Anonymous(_) => &[],
        }
    }
}