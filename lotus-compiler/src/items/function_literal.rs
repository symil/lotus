use std::slice::from_ref;

use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Type, Vasm};
use super::{Expression, Identifier, BlockExpression};

#[parsable]
pub struct FunctionLiteral {
    #[parsable(suffix="=>")]
    pub arguments: FunctionLiteralArguments,
    pub body: FunctionLiteralBody
}

#[parsable]
pub enum FunctionLiteralArguments {
    Single(Identifier),
    #[parsable(brackets="()")]
    Multiple(Vec<Identifier>)
}

#[parsable]
pub enum FunctionLiteralBody {
    Block(BlockExpression),
    Expr(Expression)
}

impl FunctionLiteral {
    pub fn process(&self, type_hint: Option<&Type>, context: &mut ProgramContext) -> Option<Vasm> {
        let arg_names = match &self.arguments {
            FunctionLiteralArguments::Single(name) => from_ref(name),
            FunctionLiteralArguments::Multiple(names) => names.as_slice(),
        };
        let arg_types = match type_hint {
            Some(Type::Function(signature)) => signature.argument_types.as_slice(),
            _ => &[]
        };

        None
    }
}