use std::str::FromStr;
use enum_as_string_macro::*;
use crate::merge;

use super::{ToInt, ToWat};

#[derive(Default)]
pub struct Wat {
    keyword: String,
    arguments: Vec<Wat>
}

impl Wat {
    pub fn new<T : ToString>(keyword: T, arguments: Vec<Wat>) -> Self {
        Self { keyword: keyword.to_string(), arguments }
    }

    pub fn single<T : ToString>(keyword: T) -> Self {
        Self { keyword: keyword.to_string(), arguments: vec![] }
    }

    pub fn push<T : ToWat>(&mut self, value: T) {
        self.arguments.push(value.to_wat())
    }

    pub fn append(&mut self, values: Vec<Wat>) {
        self.arguments.extend(values);
    }

    pub fn var_name(var_name: &str) -> Self {
        Self::single(format!("${}", var_name))
    }

    pub fn string(value: &str) -> Self {
        Self::single(format!("\"{}\"", value))
    }

    pub fn export(name: &str) -> Self {
        wat!["export", Self::string(name)]
    }

    pub fn const_i32<T : ToInt>(value: T) -> Self {
        wat!["i32.const", value.to_i32()]
    }

    pub fn import_function(namespace: &str, sub_namespace: &str, func_name: &str, params: Vec<&'static str>, result: Option<&'static str>) -> Self {
        let mut func_content = wat!["func", Self::var_name(func_name)];

        for ty in params {
            func_content.push(wat!["param", ty]);
        }

        if let Some(ty) = result {
            func_content.push(wat!["result", ty]);
        }

        wat!["import", Self::string(namespace), Self::string(sub_namespace), func_content]
    }

    pub fn global_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        wat![
            "global",
            Self::var_name(var_name),
            wat!["mut", "i32"],
            wat!["i32.const", value.to_i32()]
        ]
    }

    pub fn function(var_name: &str, export_name: Option<&str>, arguments: Vec<(&str, &'static str)>, result: Option<&'static str>, instructions: Vec<Wat>) -> Self {
        let mut func = wat!["func", Self::var_name(var_name)];

        if let Some(name) = export_name {
            func.push(Self::export(name));
        }

        for (name, ty) in arguments {
            func.push(wat!["param", Self::var_name(name), ty]);
        }

        if let Some(ty) = result {
            func.push(wat!["result", ty]);
        }

        for inst in instructions {
            func.push(inst);
        }

        func
    }

    pub fn declare_i32_local(var_name: &str) -> Self {
        wat!["local", Self::var_name(var_name), "i32"]
    }

    pub fn set_local<T : ToWat>(var_name: &str, value: T) -> Self {
        wat!["local.set", Self::var_name(var_name), value]
    }

    pub fn get_local(var_name: &str) -> Self {
        wat!["local.get", Self::var_name(var_name)]
    }

    pub fn set_i32_at_addr<T : ToWat, U : ToWat>(addr: T, value: U) -> Self {
        wat!["i32.store", addr, value]
    }

    pub fn increment_i32_local<T : ToInt>(var_name: &str, value: T) -> Self {
        Wat::set_local("stack_index", wat![
            "i32.add",
            Wat::get_local("stack_index"),
            Wat::const_i32(value)
        ])
    }

    pub fn while_loop<T : ToInt>(var_name: &str, end_value: Wat, inc_value: T, statements: Vec<Wat>) -> Wat {
        wat!["block", Wat::new("loop", merge![
            vec![
                wat!["br_if", 1, wat![ "i32.ge_s", Wat::get_local(var_name), end_value ]],
            ],
            statements,
            vec![
                Wat::increment_i32_local(var_name, inc_value),
                wat!["br", 0]
            ]
        ])]
    }

    pub fn basic_loop(condition: Wat, statements: Vec<Wat>) -> Wat {
        wat!["block", Wat::new("loop", merge![
            vec![ wat!["br_if", 1, wat!["eqz", condition]] ],
            statements,
            vec![ wat!["br", 0] ]
        ])]
    }
}

impl Wat {
    pub fn to_string(&self, indent: usize) -> String {
        if self.arguments.is_empty() {
            self.keyword.clone()
        } else {
            let items = if self.arguments.len() <= 3 {
                self.arguments.iter().map(|expr| expr.to_string(indent)).collect::<Vec<String>>().join(" ")
            } else {
                self.arguments.iter().map(|expr| format!("\n{}{}", indent_level_to_string(indent + 1), expr.to_string(indent + 1))).collect::<Vec<String>>().join("")
            };

            format!("({} {})", &self.keyword, items)
        }
    }
}

fn indent_level_to_string(level: usize) -> String {
    String::from_utf8(vec![b' '; level * 2]).unwrap()
}

#[macro_export]
macro_rules! wat {
    () => {
        Wat::default()
    };
    ($keyword:expr $(,$arg:expr)*) => {
        {
            let keyword = $keyword;
            let mut result = keyword.to_wat();
            $(
                {
                    let arg = $arg;
                    result.push(arg);
                }
            )*

            result
        }
    };
}

pub use wat;