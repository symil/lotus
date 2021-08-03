use std::str::FromStr;
use enum_as_string_macro::*;

#[derive(Debug)]
pub enum Wat {
    Keyword(Keyword),
    Variable(String),
    StringLiteral(String),
    IntLiteral(i32),
    FloatLiteral(i32),
    Type(Type),
    Mutability(Mutability),
    Instruction(Type, InstructionName),
    Composite(Vec<Wat>)
}

#[enum_as_string(lowercase)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword { Module, Memory, Global, Export, Mut, Func, Param, Result, Drop }

#[enum_as_string(lowercase)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mutability { Mut }

#[enum_as_string(lowercase)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type { I32, I64, F32, F64 }

#[enum_as_string(lowercase)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionName { Const, Load, Store }

impl Wat {
    pub fn module(mut expressions: Vec<Self>) -> Self {
        expressions.insert(0, Self::Keyword(Keyword::Module));

        Self::Composite(expressions)
    }

    pub fn export(name: &str) -> Self {
        Self::Composite(vec![
            Self::Keyword(Keyword::Export),
            Self::StringLiteral(name.to_string())
        ])
    }

    pub fn memory(name: &str, page_count: usize) -> Self {
        Self::Composite(vec![
            Self::Keyword(Keyword::Memory),
            Self::export(name),
            Self::IntLiteral(page_count as i32)
        ])
    }

    pub fn global_int(var_name: &str, value: i32) -> Self {
        let ty = Type::I32;
        
        Self::Composite(vec![
            Self::Keyword(Keyword::Global),
            Self::Variable(var_name.to_string()),
            Self::Composite(vec![
                Self::Mutability(Mutability::Mut),
                Self::Type(ty)
            ]),
            Self::Composite(vec![
                Self::Instruction(ty, InstructionName::Const),
                Self::IntLiteral(value)
            ])
        ])
    }

    pub fn function(var_name: &str, export_name: Option<&str>, arguments: Vec<(&str, &'static str)>, result: Option<&str>, instructions: Vec<Wat>) -> Self {
        Self::Composite(
            vec![
                Self::Keyword(Keyword::Func),
                Self::Variable(var_name.to_string())
            ].into_iter()
            .chain(export_name.into_iter().map(|name| Self::export(name)))
            .chain(arguments.into_iter().map(|(name, ty)| Self::Composite(vec![
                Self::Keyword(Keyword::Param),
                Self::Variable(name.to_string()),
                Self::Type(Type::from_str(ty).unwrap())
            ])))
            .chain(result.into_iter().map(|ty| Self::Composite(vec![
                Self::Keyword(Keyword::Result),
                Self::Type(Type::from_str(ty).unwrap())
            ])))
            .chain(instructions)
            .collect()
        )
    }

    pub fn instruction(ty: &str, name: &str, arguments: Vec<Self>) -> Self {
        Self::Composite(
            vec![Self::Instruction(Type::from_str(ty).unwrap(), InstructionName::from_str(name).unwrap())].into_iter()
                .chain(arguments)
                .collect()
        )
    }

    pub fn const_i32(value: i32) -> Self {
        Self::Composite(vec![
            Self::Instruction(Type::I32, InstructionName::Const),
            Self::IntLiteral(value)
        ])
    }

    pub fn drop() -> Self {
        Self::Keyword(Keyword::Drop)
    }
}

impl ToString for Wat {
    fn to_string(&self) -> String {
        match self {
            Wat::Keyword(keyword) => keyword.to_string(),
            Wat::Variable(name) => format!("${}", name),
            Wat::StringLiteral(value) => format!("\"{}\"", value),
            Wat::IntLiteral(value) => format!("{}", value),
            Wat::FloatLiteral(value) => format!("{}", value),
            Wat::Type(ty) => ty.to_string(),
            Wat::Mutability(mutability) => mutability.to_string(),
            Wat::Instruction(ty, name) => format!("{}.{}", ty.to_string(), name.to_string()),
            Wat::Composite(expressions) => {
                if expressions.len() <= 4 {
                    format!("({})", expressions.iter().map(|expr| expr.to_string()).collect::<Vec<String>>().join(" "))
                } else {
                    let items = expressions.iter().map(|expr| expr.to_string()).collect::<Vec<String>>().join("\n  ");

                    format!("({})", items)
                }
            },
        }
    }
}