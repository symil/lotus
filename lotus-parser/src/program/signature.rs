use core::{fmt};
use colored::Colorize;
use super::{Type, display_join};

pub struct Signature {
    pub arguments: Vec<Type>,
    pub return_value: Option<Type>
}

impl Signature {
    pub fn replace_parameters(&self, this_type: Option<&Type>, function_parameters: &[Type]) -> Self {
        Self {
            arguments: self.arguments.iter().map(|ty| ty.replace_parameters(this_type, function_parameters)).collect(),
            return_value: self.return_value.as_ref().and_then(|ty| Some(ty.replace_parameters(this_type, function_parameters))),
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

        if let Some(ret) = &self.return_value {
            s.push_str(&format!(" -> {}", ret));
        }

        write!(f, "{}", s.bold())
    }
}