use std::{ops::Deref, str::FromStr};
use crate::{wat};
use super::{ToInt, ToWat, ToWatVec};

#[derive(Default, Debug, Clone)]
pub struct Wat {
    pub keyword: String,
    pub arguments: Vec<Wat>
}

impl Wat {
    pub fn from<T : ToWat>(value: T) -> Self {
        value.to_wat()
    }

    pub fn new<T : ToString, V : ToWatVec>(keyword: T, arguments: V) -> Self {
        Self { keyword: keyword.to_string(), arguments: arguments.to_wat_vec() }
    }

    pub fn single<T : ToString>(keyword: T) -> Self {
        Self { keyword: keyword.to_string(), arguments: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.keyword.is_empty()
    }

    pub fn push<T : ToWat>(&mut self, value: T) {
        self.arguments.push(value.to_wat())
    }

    pub fn extend<T : ToWatVec>(&mut self, values: T) {
        self.arguments.extend(values.to_wat_vec());
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

    pub fn placeholder(value: &str) -> Self {
        Self::single(format!("#{}", value))
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

    pub fn call<S : Deref<Target=str>, T : ToWatVec>(func_name: S, arguments: T) -> Self {
        wat!["call", Wat::var_name(&func_name), arguments]
    }

    pub fn call_from_stack(func_name: &str) -> Self {
        wat!["call", Wat::var_name(func_name)]
    }

    // GLOBALS

    pub fn declare_global(var_name: &str, ty: &'static str) -> Self {
        let init_name = match ty {
            "i32" => "i32.const",
            "f32" => "f32.const",
            "i64" => "i64.const",
            "f64" => "f64.const",
            _ => unreachable!()
        };

        wat!["global", Self::var_name(var_name), wat!["mut", ty], wat![init_name, 0]]
    }

    pub fn declare_global_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        wat!["global", Self::var_name(var_name), wat!["mut", "i32"], wat!["i32.const", value.to_i32()]]
    }

    pub fn declare_global_f32(var_name: &str, value: f32) -> Self {
        wat!["global", Self::var_name(var_name), wat!["mut", "f32"], wat!["f32.const", value]]
    }

    pub fn set_global<T : ToWat>(var_name: &str, value: T) -> Self {
        wat!["global.set", Self::var_name(var_name), value]
    }
    
    pub fn set_global_from_stack(var_name: &str) -> Self {
        wat!["global.set", Self::var_name(var_name)]
    }

    pub fn get_global(var_name: &str) -> Self {
        wat!["global.get", Self::var_name(var_name)]
    }

    pub fn tee_global_from_stack(var_name: &str) -> Self {
        wat!["global.tee", Self::var_name(var_name)]
    }

    pub fn increment_global_i32<T : ToInt>(var_name: &str, value: T) -> Self {
        Wat::set_global(var_name, wat![
            "i32.add",
            Wat::get_global(var_name),
            Wat::const_i32(value)
        ])
    }

    // LOCALS

    pub fn declare_local(var_name: &str, ty: &'static str) -> Self {
        wat!["local", Self::var_name(var_name), ty]
    }

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

    pub fn tee_local_from_stack(var_name: &str) -> Self {
        wat!["local.tee", Self::var_name(var_name)]
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

    pub fn import_function(file_namespace: &str, sub_file_namespace: &str, func_name: &str, params: Vec<&'static str>, result: Option<&'static str>) -> Self {
        let mut func_content = wat!["func", Self::var_name(func_name)];

        for ty in params {
            func_content.push(wat!["param", ty]);
        }

        if let Some(ty) = result {
            func_content.push(wat!["result", ty]);
        }

        wat!["import", Self::string(file_namespace), Self::string(sub_file_namespace), func_content]
    }

    pub fn declare_function(var_name: &str, export_name: Option<&str>, arguments: Vec<(&str, &str)>, results: Vec<&str>, locals: Vec<(&str, &str)>, instructions: Vec<Wat>) -> Self {
        let mut func = wat!["func", Self::var_name(var_name)];

        if let Some(name) = export_name {
            func.push(Self::export(name));
        }

        for (name, ty) in arguments {
            func.push(wat!["param", Self::var_name(name), ty]);
        }

        for ty in results {
            func.push(wat!["result", ty]);
        }

        for (name, ty) in locals {
            func.push(wat!["local", Self::var_name(name), ty]);
        }

        for inst in instructions {
            func.push(inst);
        }

        func
    }

    pub fn declare_function_type(type_name: &str, arguments: &[&str], results: &[&str]) -> Wat {
        let mut body = wat!["func"];

        for arg_type in arguments {
            body.push(wat!["param", *arg_type]);
        }

        for result_type in results {
            body.push(wat!["result", *result_type]);
        }

        wat!["type", Self::var_name(type_name), body]
    }

    pub fn while_loop<T : ToWatVec>(condition: Wat, statements: T) -> Wat {
        wat!["block", wat!["loop",
            wat!["br_if", 1, wat!["i32.eqz", condition]],
            statements,
            wat!["br", 0]
        ]]
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

    // OTHER
    
    pub fn replace(&mut self, pattern: &str, replacement: &str) {
        self.keyword = self.keyword.replace(pattern, replacement);

        for arg in &mut self.arguments.iter_mut() {
            arg.replace(pattern, replacement);
        }
    }

    pub fn replace_placeholder<F : Fn(&str) -> String>(&mut self, f: &F) {
        if self.keyword.starts_with("#") {
            self.keyword = f(&self.keyword[1..]);
        }

        for arg in &mut self.arguments.iter_mut() {
            arg.replace_placeholder(f);
        }
    }

    pub fn to_string(&self, indent: usize) -> String {
        if self.arguments.is_empty() {
            let wrap = match self.keyword.as_str() {
                "func" => true,
                "block" => true,
                _ => self.keyword.contains(".") && (!is_number_char(self.keyword.chars().next().unwrap()) && !self.keyword.starts_with("memory"))
            };

            match wrap {
                true => format!("({})", self.keyword),
                false => self.keyword.clone()
            }
        } else {
            let split_into_lines = match self.keyword.as_str() {
                "block" => true,
                _ => self.arguments.len() > 3
            };

            let items = match split_into_lines {
                true => self.arguments.iter().map(|expr| format!("\n{}{}", indent_level_to_string(indent + 1), expr.to_string(indent + 1))).collect::<Vec<String>>().join(""),
                false => self.arguments.iter().map(|expr| expr.to_string(indent)).collect::<Vec<String>>().join(" ")
            };

            match split_into_lines {
                true => format!("({} {}\n{})", &self.keyword, items, indent_level_to_string(indent)),
                false => format!("({} {})", &self.keyword, items),
            }
        }
    }
}

fn indent_level_to_string(level: usize) -> String {
    String::from_utf8(vec![b' '; level * 2]).unwrap()
}

fn is_number_char(c: char) -> bool {
    c == '-' || c.is_numeric()
}