use std::ops::Deref;

use parsable::parsable;

use super::{boolean_literal::BooleanLiteral, identifier::Identifier, number_literal::NumberLiteral, string_literal::StringLiteral};

pub type Expression = Operation;

#[parsable]
pub struct Operation {
    pub first: Operand,
    pub others: Vec<(BinaryOperator, Operand)>
}

#[parsable]
pub enum Operand {
    // TODO: add anonymous function
    #[parsable(brackets="()")]
    Parenthesized(Box<Operation>),
    VoidLiteral = ";",
    NullLiteral = "null",
    BooleanLiteral(BooleanLiteral),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    UnaryOperation(Box<UnaryOperation>),
    VarPath(VarPath),
}

#[parsable]
pub struct ObjectLiteral {
    pub type_name: Identifier,
    #[parsable(brackets="{}", separator=",")]
    pub fields: Vec<FieldInitialization>
}

#[parsable]
pub struct FieldInitialization {
    pub name: Identifier,
    pub value: Expression
}

#[parsable]
pub struct ArrayLiteral {
    #[parsable(brackets="[]", separator=",")]
    pub items: Vec<Expression>
}

#[parsable]
pub struct VarPath {
    pub prefix: Option<VarPrefix>,
    pub name: Identifier,
    pub path: Vec<PathSegment>
}

#[parsable(impl_display=true)]
#[derive(PartialEq, Copy)]
pub enum VarPrefix {
    This = "#",
    Payload = "$"
}

#[parsable]
pub enum PathSegment {
    #[parsable(prefix=".")]
    FieldAccess(Identifier),
    #[parsable(brackets="[]")]
    BracketIndexing(Operation),
    #[parsable(brackets="()", sep=",")]
    FunctionCall(ArgumentList)
}

impl PathSegment {
    pub fn is_function_call(&self) -> bool {
        match self {
            PathSegment::FunctionCall(_) => true,
            _ => false
        }
    }
}

#[parsable]
pub struct ArgumentList {
    #[parsable(brackets="()", sep=",")]
    pub list: Vec<Operation>
}

#[parsable(impl_display=true)]
pub enum UnaryOperator {
    Not = "!",
    Plus = "+",
    Minus = "-"
}

#[parsable]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Operand
}

#[parsable(impl_display=true)]
pub enum BinaryOperator {
    Plus = "+",
    Minus = "-",
    Mult = "*",
    Div = "/",
    Mod = "%",
    And = "&&",
    Or = "||",
    Eq = "==",
    Neq = "!=",
    Gte = ">=",
    Gt = ">",
    Lte = "<=",
    Lt = "<",
    Range = ".."
}

#[parsable]
pub struct TernaryOperation {
    pub condition: Operation,
    #[parsable(prefix="?")]
    pub if_expr: Operation,
    #[parsable(prefix=":")]
    pub else_expr: Operation
}

impl Deref for ArgumentList {
    type Target = Vec<Operation>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}