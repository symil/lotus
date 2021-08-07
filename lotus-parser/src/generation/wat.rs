use std::str::FromStr;
use enum_as_string_macro::*;
use crate::{wat, merge};
use super::{ToInt, ToWat, ToWatVec};

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

    pub fn extend(&mut self, values: Vec<Wat>) {
        self.arguments.extend(values);
    }

    pub fn var_name(var_name: &str) -> Self {
        Self::single(format!("${}", var_name))
    }

    pub fn string(value: &str) -> Self {
        Self::single(format!("\"{}\"", value))
    }

    pub fn inst(name: &str) -> Self {
        wat![name]
    }

    // BASIC

    pub fn nop() -> Self {
        wat!["nop"]
    }

    pub fn export(name: &str) -> Self {
        wat!["export", Self::string(name)]
    }

    pub fn const_i32<T : ToInt>(value: T) -> Self {
        wat!["i32.const", value.to_i32()]
    }

    pub fn const_f32(value: f32) -> Self {
        wat!["f32.const", value]
    }

    pub fn call<T : ToWatVec>(func_name: &str, arguments: T) -> Self {
        wat!["call", Wat::var_name(func_name), arguments]
    }

    pub fn call_no_arg(func_name: &str) -> Self {
        wat!["call", Wat::var_name(func_name)]
    }

    // GLOBALS

    pub fn declare_global_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        wat!["global", Self::var_name(var_name), wat!["mut", "i32"], wat!["i32.const", value.to_i32()]]
    }

    pub fn set_global<T : ToWat>(var_name: &str, value: T) -> Self {
        wat!["global.set", Self::var_name(var_name), value]
    }

    pub fn get_global(var_name: &str) -> Self {
        wat!["global.get", Self::var_name(var_name)]
    }

    pub fn increment_global_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        Wat::set_global(var_name, wat![
            "i32.add",
            Wat::get_global(var_name),
            Wat::const_i32(value)
        ])
    }

    // LOCALS

    pub fn declare_local_i32(var_name: &str) -> Self {
        wat!["local", Self::var_name(var_name), "i32"]
    }

    pub fn set_local<T : ToWat>(var_name: &str, value: T) -> Self {
        wat!["local.set", Self::var_name(var_name), value]
    }

    pub fn set_local_from_stack(var_name: &str) -> Self {
        wat!["local.set", Self::var_name(var_name)]
    }

    pub fn get_local(var_name: &str) -> Self {
        wat!["local.get", Self::var_name(var_name)]
    }

    pub fn increment_local_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        Wat::set_local(var_name, wat![
            "i32.add",
            Wat::get_local(var_name),
            Wat::const_i32(value)
        ])
    }

    // MEMORY

    pub fn mem_set_i32<T : ToWat>(local_var_name: &str, value: T) -> Self {
        wat!["i32.store", Wat::get_local(local_var_name), value]
    }

    pub fn mem_set_i32_with_offset<O : ToInt, V : ToWat>(local_var_name: &str, offset: O, value: V) -> Self {
        wat!["i32.store", wat!["i32.add", Wat::get_local(local_var_name), Wat::const_i32(offset)], value]
    }

    pub fn mem_get_i32(local_var_name: &str) -> Self {
        wat!["i32.load", Wat::get_local(local_var_name)]
    }

    pub fn mem_get_i32_with_offset<O : ToInt>(local_var_name: &str, offset: O) -> Self {
        wat!["i32.load", wat!["i32.add", Wat::get_local(local_var_name), Wat::const_i32(offset)]]
    }

    // COMPLEX BLOCKS

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

    pub fn declare_function(var_name: &str, export_name: Option<&str>, arguments: Vec<(&str, &'static str)>, result: Option<&'static str>, instructions: Vec<Wat>) -> Self {
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

    pub fn while_loop<T : ToWatVec>(condition: Wat, statements: T) -> Wat {
        wat!["block", Wat::new("loop", merge![
            wat!["br_if", 1, wat!["i32.eqz", condition]],
            statements,
            wat!["br", 0]
        ])]
    }

    pub fn if_else<T : ToWatVec, U : ToWatVec>(condition: Wat, if_block: T, else_block: U) -> Wat {
        wat!["block",
            wat!["block",
                wat!["br_if", 0, wat!["i32.eqz", condition]],
                if_block,
                wat!["br", 1]
            ],
            else_block
        ]
    }

    // LOG

    pub fn log_var(var_name: &str) -> Wat {
        Wat::call("log_i32", Wat::get_local(var_name))
    }

    pub fn log_addr(var_name: &str) -> Wat {
        Wat::call("log_i32", Wat::mem_get_i32(var_name))
    }

    pub fn to_string(&self, indent: usize) -> String {
        if self.arguments.is_empty() {
            if self.keyword.contains(".") {
                format!("({})", self.keyword)
            } else {
                self.keyword.clone()
            }
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