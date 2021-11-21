use core::{fmt};
use std::rc::Rc;
use colored::Colorize;
use crate::program::BuiltinType;
use super::{ProgramContext, Type, TypeIndex, TypeInstanceHeader, display_join};

#[derive(Debug, Clone)]
pub struct Signature {
    pub this_type: Option<Type>,
    pub argument_types: Vec<Type>,
    pub return_type: Type
}

pub struct ResolvedSignature {
    pub this_type: Option<Rc<TypeInstanceHeader>>,
    pub argument_types: Vec<Rc<TypeInstanceHeader>>,
    pub return_type: Rc<TypeInstanceHeader>
}

impl Signature {
    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Self {
        Self {
            this_type: self.this_type.as_ref().map(|ty| ty.replace_parameters(this_type, function_parameters)),
            argument_types: self.argument_types.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            return_type: self.return_type.replace_parameters(this_type, function_parameters)
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = format!("fn({})", display_join(&self.argument_types, ", "));

        if !self.return_type.is_void() {
            s.push_str(&format!(" -> {}", &self.return_type));
        }

        s
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> ResolvedSignature {
        ResolvedSignature {
            this_type: self.this_type.as_ref().map(|ty| ty.resolve(type_index, context)),
            argument_types: self.argument_types.iter().map(|ty| ty.resolve(type_index, context)).collect(),
            return_type: self.return_type.resolve(type_index, context),
        }
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.argument_types == other.argument_types && self.return_type == other.return_type
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string().bold())
    }
}