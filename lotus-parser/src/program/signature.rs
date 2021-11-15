use core::{fmt};
use colored::Colorize;
use crate::program::BuiltinType;

use super::{Type, display_join};

pub struct Signature {
    pub arguments: Vec<Type>,
    pub return_value: Type
}

impl Signature {
    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Self {
        Self {
            arguments: self.arguments.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            return_value: self.return_value.replace_parameters(this_type, function_parameters)
        }
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments && self.return_value == other.return_value
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("fn({})", display_join(&self.arguments, ", "));

        if self.return_value.is_builtin_type(BuiltinType::Void) {
            s.push_str(&format!(" -> {}", &self.return_value));
        }

        write!(f, "{}", s.bold())
    }
}