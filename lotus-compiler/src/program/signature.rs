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

    pub fn check_type_parameters(&self, context: &mut ProgramContext) {
        for arg_type in &self.argument_types {
            arg_type.check_parameters(context);
        }

        self.return_type.check_parameters(context);
    }

    pub fn to_string(&self) -> String {
        // let this_str = match &self.this_type {
        //     Some(ty) => format!("[{}]", ty.to_string()),
        //     None => format!(""),
        // };
        // let mut s = format!("fn{}({})", this_str, display_join(&self.argument_types, ", "));
        let mut s = format!("fn({})", display_join(&self.argument_types, ", "));

        if !self.return_type.is_void() {
            s.push_str(&format!("({})", &self.return_type));
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

    pub fn is_assignable_to(&self, other: &Self) -> bool {
        if self.argument_types.len() != other.argument_types.len() {
            return false;
        }

        let this_ok = match (&self.this_type, &other.this_type) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(self_this), Some(other_this)) => other_this.is_assignable_to(self_this),
        };

        if !this_ok {
            return false;
        }

        for (self_arg, other_arg) in self.argument_types.iter().zip(other.argument_types.iter()) {
            if !other_arg.is_assignable_to(self_arg) {
                return false;
            }
        }

        if !other.return_type.is_assignable_to(&self.return_type) {
            return false;
        }

        true
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self {
            this_type: None,
            argument_types: vec![],
            return_type: Type::Undefined
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