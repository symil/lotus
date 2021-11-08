use std::rc::Rc;
use parsable::DataLocation;

use crate::{items::{Identifier, make_string_value_from_literal}, program::{DUPLICATE_INT_WASM_FUNC_NAME, FieldKind, FunctionInstanceParameters, GeneratedItemIndex, ItemGenerator, MEMORY_CELL_BYTE_SIZE, OBJECT_HEADER_SIZE, SWAP_FLOAT_INT_WASM_FUNC_NAME, SWAP_INT_INT_WASM_FUNC_NAME, TypeInstanceParameters}, utils::Link, wat};
use super::{FunctionBlueprint, ProgramContext, ToInt, ToVasm, Type, TypeBlueprint, TypeIndex, VariableInfo, VariableKind, Vasm, Wat, function_blueprint};

pub type VI = VirtualInstruction;

#[derive(Debug, Clone)]
pub enum VirtualInstruction {
    Drop,
    Eqz,
    Raw(Wat),
    IntConstant(i32),
    FloatConstant(f32),
    TypeId(Type),
    TypeName(Type),
    Store(VirtualStashInfo),
    Load(VirtualStashInfo),
    GetVariable(VirtualGetVariableInfo),
    SetVariable(VirtualSetVariableInfo),
    TeeVariable(VirtualSetVariableInfo),
    GetField(VirtualGetFieldInfo),
    SetField(VirtualSetFieldInfo),
    FunctionCall(VirtualFunctionCallInfo),
    Loop(VirtualLoopInfo),
    Block(VirtualBlockInfo),
    Jump(VirtualJumpInfo),
    JumpIf(VirtualJumpIfInfo),
    IfThenElse(IfThenElseInfo)
}

#[derive(Debug, Clone)]
pub struct VirtualStashInfo {
    pub value_type: Type,
    pub wasm_var_name: String
}

#[derive(Debug, Clone)]
pub struct VirtualGetVariableInfo {
    pub var_info: Rc<VariableInfo>,
}

#[derive(Debug, Clone)]
pub struct VirtualSetVariableInfo {
    pub var_info: Rc<VariableInfo>,
    pub value: Option<Vasm>
}

#[derive(Debug, Clone)]
pub struct VirtualCreateObjectInfo {
    pub object_type: Type,
}

#[derive(Debug, Clone)]
pub struct VirtualGetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize
}

#[derive(Debug, Clone)]
pub struct VirtualSetFieldInfo {
    pub field_type: Type,
    pub field_offset: usize,
    pub value: Option<Vasm>
}

#[derive(Debug, Clone)]
pub struct VirtualFunctionCallInfo {
    pub caller_type: Option<Type>,
    pub function: Link<FunctionBlueprint>,
    pub parameters: Vec<Type>,
    pub dynamic_methods_index_var: Option<Rc<VariableInfo>>,
    pub args: Vasm,
}

#[derive(Debug, Clone)]
pub struct VirtualJumpInfo {
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct VirtualJumpIfInfo {
    pub depth: u32,
    pub condition: Option<Vasm>,
}

#[derive(Debug, Clone)]
pub struct VirtualLoopInfo {
    pub content: Vasm,
}

#[derive(Debug, Clone)]
pub struct VirtualBlockInfo {
    pub result: Vec<Type>,
    pub content: Vasm,
}

#[derive(Debug, Clone)]
pub struct IfThenElseInfo {
    pub return_type: Option<Type>,
    pub condition: Vasm,
    pub then_branch: Vasm,
    pub else_branch: Vasm
}

impl VirtualInstruction {
    pub fn raw(value: Wat) -> Self {
        Self::Raw(value)
    }

    pub fn int<T : ToInt>(value: T) -> Self {
        Self::IntConstant(value.to_i32())
    }

    pub fn float(value: f32) -> Self {
        Self::FloatConstant(value)
    }

    pub fn type_id(ty: &Type) -> Self {
        Self::TypeId(ty.clone())
    }

    pub fn type_name(ty: &Type) -> Self {
        Self::TypeName(ty.clone())
    }

    pub fn store(value_type: &Type, id: u64) -> Self {
        Self::Store(VirtualStashInfo {
            value_type: value_type.clone(),
            wasm_var_name: format!("tmp_{}", id),
        })
    }

    pub fn load(value_type: &Type, id: u64) -> Self {
        Self::Load(VirtualStashInfo {
            value_type: value_type.clone(),
            wasm_var_name: format!("tmp_{}", id),
        })
    }

    pub fn get_var(var_info: &Rc<VariableInfo>) -> Self {
        Self::GetVariable(VirtualGetVariableInfo{
            var_info: Rc::clone(var_info),
        })
    }

    pub fn set_var<T : ToVasm>(var_info: &Rc<VariableInfo>, value: T) -> Self {
        Self::SetVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: Some(value.to_vasm())
        })
    }

    pub fn set_var_from_stack(var_info: &Rc<VariableInfo>) -> Self {
        Self::SetVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: None
        })
    }

    pub fn tee_var<T : ToVasm>(var_info: &Rc<VariableInfo>, value: T) -> Self {
        Self::TeeVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: Some(value.to_vasm())
        })
    }

    pub fn tee_var_from_stack(var_info: &Rc<VariableInfo>) -> Self {
        Self::TeeVariable(VirtualSetVariableInfo {
            var_info: Rc::clone(var_info),
            value: None
        })
    }

    pub fn call_function<T : ToVasm>(function: Link<FunctionBlueprint>, parameters: &[Type], args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            caller_type: None,
            function,
            parameters: parameters.to_vec(),
            dynamic_methods_index_var: None,
            args: args.to_vasm(),
        })
    }

    pub fn call_method<T : ToVasm>(caller_type: &Type, function: Link<FunctionBlueprint>, parameters: &[Type], dynamic_methods_index_var: Option<Rc<VariableInfo>>, args: T) -> Self {
        Self::FunctionCall(VirtualFunctionCallInfo {
            caller_type: Some(caller_type.clone()),
            function,
            parameters: parameters.to_vec(),
            dynamic_methods_index_var,
            args: args.to_vasm(),
        })
    }

    pub fn call_regular_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        // println!("{}: {}", caller_type, method_name);
        let function = caller_type.get_regular_method(method_name, context).unwrap().function.clone();

        Self::call_method(caller_type, function, parameters, None, args)
    }
    
    pub fn call_static_method<T : ToVasm>(caller_type: &Type, method_name: &str, parameters: &[Type], args: T, context: &ProgramContext) -> Self {
        // println!("{}: {}", caller_type, method_name);
        let function = caller_type.get_static_method(method_name, context).unwrap().function.clone();

        Self::call_method(caller_type, function, parameters, None, args)
    }

    pub fn get_field(field_type: &Type, field_offset: usize) -> Self {
        Self::GetField(VirtualGetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
        })
    }

    pub fn set_field(field_type: &Type, field_offset: usize, value: Vasm) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
            value: Some(value)
        })
    }

    pub fn set_field_from_stack(field_type: &Type, field_offset: usize) -> Self {
        Self::SetField(VirtualSetFieldInfo {
            field_type: field_type.clone(),
            field_offset,
            value: None
        })
    }

    pub fn loop_<T : ToVasm>(content: T) -> Self {
        Self::Loop(VirtualLoopInfo {
            content: content.to_vasm(),
        })
    }

    pub fn block<T : ToVasm>(content: T) -> Self {
        Self::Block(VirtualBlockInfo {
            result: vec![],
            content: content.to_vasm(),
        })
    }

    pub fn typed_block<T : ToVasm>(result: Vec<Type>, content: T) -> Self {
        Self::Block(VirtualBlockInfo {
            result,
            content: content.to_vasm(),
        })
    }

    pub fn jump(depth: u32) -> Self {
        Self::Jump(VirtualJumpInfo {
            depth
        })
    }

    pub fn jump_if<T : ToVasm>(depth: u32, condition: T) -> Self {
        Self::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: Some(condition.to_vasm()),
        })
    }

    pub fn jump_if_from_stack(depth: u32) -> Self {
        Self::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: None
        })
    }

    pub fn if_then_else(return_type: Option<&Type>, condition: Vasm, then_branch: Vasm, else_branch: Vasm) -> Self {
        Self::IfThenElse(IfThenElseInfo {
            return_type: return_type.cloned(),
            condition,
            then_branch,
            else_branch,
        })
    }

    pub fn replace_type_parameters(&self, this_type: &Type, id: u64) -> Self {
        match self {
            VirtualInstruction::Drop => VirtualInstruction::Drop,
            VirtualInstruction::Eqz => VirtualInstruction::Eqz,
            VirtualInstruction::Raw(wat) => VirtualInstruction::Raw(wat.clone()),
            VirtualInstruction::IntConstant(value) => VirtualInstruction::IntConstant(value.clone()),
            VirtualInstruction::FloatConstant(value) => VirtualInstruction::FloatConstant(value.clone()),
            VirtualInstruction::TypeId(ty) => VirtualInstruction::TypeId(ty.replace_parameters(Some(this_type), &[])),
            VirtualInstruction::TypeName(ty) => VirtualInstruction::TypeId(ty.replace_parameters(Some(this_type), &[])),
            VirtualInstruction::Store(info) => VirtualInstruction::Store(VirtualStashInfo {
                value_type: info.value_type.replace_parameters(Some(this_type), &[]),
                wasm_var_name: info.wasm_var_name.clone(),
            }),
            VirtualInstruction::Load(info) => VirtualInstruction::Load(VirtualStashInfo {
                value_type: info.value_type.replace_parameters(Some(this_type), &[]),
                wasm_var_name: info.wasm_var_name.clone(),
            }),
            VirtualInstruction::GetVariable(info) => VirtualInstruction::GetVariable(VirtualGetVariableInfo {
                var_info: Rc::new(info.var_info.replace_type_parameters(this_type, id)),
            }),
            VirtualInstruction::SetVariable(info) => VirtualInstruction::SetVariable(VirtualSetVariableInfo {
                var_info: Rc::new(info.var_info.replace_type_parameters(this_type, id)),
                value: info.value.as_ref().and_then(|value| Some(value.replace_type_parameters(this_type, id))),
            }),
            VirtualInstruction::TeeVariable(info) => VirtualInstruction::TeeVariable(VirtualSetVariableInfo {
                var_info: Rc::new(info.var_info.replace_type_parameters(this_type, id)),
                value: info.value.as_ref().and_then(|value| Some(value.replace_type_parameters(this_type, id))),
            }),
            VirtualInstruction::GetField(info) => VirtualInstruction::GetField(VirtualGetFieldInfo {
                field_type: info.field_type.replace_parameters(Some(this_type), &[]),
                field_offset: info.field_offset,
            }),
            VirtualInstruction::SetField(info) => VirtualInstruction::SetField(VirtualSetFieldInfo {
                field_type: info.field_type.replace_parameters(Some(this_type), &[]),
                field_offset: info.field_offset,
                value: info.value.as_ref().and_then(|value| Some(value.replace_type_parameters(this_type, id))),
            }),
            VirtualInstruction::FunctionCall(info) => VirtualInstruction::FunctionCall(VirtualFunctionCallInfo {
                caller_type: info.caller_type.as_ref().and_then(|ty| Some(ty.replace_parameters(Some(this_type), &[]))),
                function: info.function.clone(),
                parameters: info.parameters.iter().map(|ty| ty.replace_parameters(Some(this_type), &[])).collect(),
                dynamic_methods_index_var: info.dynamic_methods_index_var.as_ref().and_then(|var_info| Some(Rc::new(var_info.replace_type_parameters(this_type, id)))),
                args: info.args.replace_type_parameters(this_type, id)
            }),
            VirtualInstruction::Loop(info) => VirtualInstruction::Loop(VirtualLoopInfo {
                content: info.content.replace_type_parameters(this_type, id),
            }),
            VirtualInstruction::Block(info) => VirtualInstruction::Block(VirtualBlockInfo {
                result: info.result.iter().map(|ty| ty.replace_parameters(Some(this_type), &[])).collect(),
                content: info.content.replace_type_parameters(this_type, id),
            }),
            VirtualInstruction::Jump(info) => VirtualInstruction::Jump(VirtualJumpInfo {
                depth: info.depth.clone(),
            }),
            VirtualInstruction::JumpIf(info) => VirtualInstruction::JumpIf(VirtualJumpIfInfo {
                depth: info.depth.clone(),
                condition: info.condition.as_ref().and_then(|value| Some(value.replace_type_parameters(this_type, id))),
            }),
            VirtualInstruction::IfThenElse(info) => VirtualInstruction::IfThenElse(IfThenElseInfo {
                return_type: info.return_type.as_ref().and_then(|ty| Some(ty.replace_parameters(Some(this_type), &[]))),
                condition: info.condition.replace_type_parameters(this_type, id),
                then_branch: info.then_branch.replace_type_parameters(this_type, id),
                else_branch: info.else_branch.replace_type_parameters(this_type, id),
            }),
        }
    }

    pub fn collect_variables(&self, list: &mut Vec<Rc<VariableInfo>>) {
        match self {
            VirtualInstruction::Drop => {},
            VirtualInstruction::Eqz => {},
            VirtualInstruction::Raw(_) => {},
            VirtualInstruction::IntConstant(_) => {},
            VirtualInstruction::FloatConstant(_) => {},
            VirtualInstruction::TypeId(_) => {},
            VirtualInstruction::TypeName(_) => {},
            VirtualInstruction::Store(info) => {
                list.push(VariableInfo::from_wasm_name(info.wasm_var_name.to_string(), info.value_type.clone(), VariableKind::Local))
            },
            VirtualInstruction::Load(_) => {},
            VirtualInstruction::GetVariable(_) => {},
            VirtualInstruction::SetVariable(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::TeeVariable(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::GetField(_) => {},
            VirtualInstruction::SetField(info) => info.value.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::FunctionCall(info) => {
                info.dynamic_methods_index_var.iter().for_each(|var_info| list.push(var_info.clone()));
                info.args.collect_variables(list);
            },
            VirtualInstruction::Loop(info) => info.content.collect_variables(list),
            VirtualInstruction::Block(info) => info.content.collect_variables(list),
            VirtualInstruction::Jump(_) => {},
            VirtualInstruction::JumpIf(info) => info.condition.iter().for_each(|vasm| vasm.collect_variables(list)),
            VirtualInstruction::IfThenElse(info) => {
                info.condition.collect_variables(list);
                info.then_branch.collect_variables(list);
                info.else_branch.collect_variables(list);
            },
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => vec![wat!["drop"]],
            VirtualInstruction::Eqz => vec![wat!["i32.eqz"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(value) => vec![Wat::const_i32(*value)],
            VirtualInstruction::FloatConstant(value) => vec![Wat::const_f32(*value)],
            VirtualInstruction::TypeId(ty) => {
                let type_instance = ty.resolve(type_index, context);

                vec![Wat::const_i32(type_instance.dynamic_method_table_offset)]
            },
            VirtualInstruction::TypeName(ty) => {
                let type_instance = ty.resolve(type_index, context);
                let vasm = make_string_value_from_literal(None, &type_instance.ty.get_name(), context).unwrap();

                vasm.resolve(type_index, context)
            },
            VirtualInstruction::GetVariable(info) => vec![info.var_info.get_to_stack()],
            VirtualInstruction::Store(info) => {
                vec![Wat::set_local_from_stack(&info.wasm_var_name)]
            },
            VirtualInstruction::Load(info) => {
                vec![Wat::get_local(&info.wasm_var_name)]
            },
            VirtualInstruction::SetVariable(info) | VirtualInstruction::TeeVariable(info) => {
                let mut content = vec![];

                if let Some(vasm) = &info.value {
                    content.extend(vasm.resolve(type_index, context));
                }

                if let VirtualInstruction::TeeVariable(_) = self {
                    content.push(info.var_info.tee_from_stack());
                } else {
                    content.push(info.var_info.set_from_stack());
                }

                content
            },
            VirtualInstruction::GetField(info) => {
                let field_type = info.field_type.resolve(type_index, context);
                
                let content = vec![
                    wat!["i32.add", Wat::const_i32(info.field_offset)],
                    wat!["i32.mul", Wat::const_i32(4)],
                    wat![format!("{}.load", field_type.wasm_type.unwrap())]
                ];

                content
            },
            VirtualInstruction::SetField(info) => {
                let mut content = vec![];
                let field_type = info.field_type.resolve(type_index, context);
                let field_wasm_type = field_type.wasm_type.unwrap();

                content.push(wat!["i32.add", Wat::const_i32(info.field_offset)]);
                content.push(wat!["i32.mul", Wat::const_i32(4)]);

                if let Some(init_value) = &info.value {
                    content.extend(init_value.resolve(type_index, context));
                } else {
                    let swap_func_name = match field_wasm_type {
                        "i32" => SWAP_INT_INT_WASM_FUNC_NAME,
                        "f32" => SWAP_FLOAT_INT_WASM_FUNC_NAME,
                        _ => unreachable!()
                    };

                    content.push(Wat::call_from_stack(swap_func_name));
                }

                content.push(wat![format!("{}.store", field_wasm_type)]);

                content
            },
            VirtualInstruction::FunctionCall(info) => {
                let mut content = vec![];

                let this_type = info.caller_type.as_ref().and_then(|ty| Some(ty.resolve(type_index, context)));
                let function_blueprint = info.function.with_ref(|function_unwrapped| {
                    match function_unwrapped.owner_interface.is_none() {
                        true => info.function.clone(),
                        false => this_type.as_ref().unwrap().get_method(function_unwrapped.get_method_kind(), function_unwrapped.name.as_str()).unwrap(),
                    }
                });
                let is_dynamic = function_blueprint.borrow().is_dynamic();
                let mut dynamic_call = false;

                if is_dynamic && this_type.as_ref().unwrap().wasm_type.contains(&"i32") {
                    if let Some(dynamic_methods_index_var) = &info.dynamic_methods_index_var {
                        let method_offset = function_blueprint.borrow().dynamic_index;
                        let func_wasm_type_name = this_type.as_ref().unwrap().get_placeholder_function_wasm_type_name(&function_blueprint);

                        content.extend(vec![
                            Wat::call_from_stack(DUPLICATE_INT_WASM_FUNC_NAME),
                            wat![Wat::const_i32(4)],
                            wat!["i32.mul"],
                            wat!["i32.load"],
                            Wat::set_local_from_stack(&dynamic_methods_index_var.wasm_name)
                        ]);
                        content.extend(info.args.resolve(type_index, context));
                        content.extend(vec![
                            dynamic_methods_index_var.get_to_stack(),
                            Wat::const_i32(method_offset),
                            wat!["i32.add"],
                            wat!["call_indirect", wat!["type", Wat::placeholder(&func_wasm_type_name)]]
                        ]);
                        dynamic_call = true;
                    }
                }

                if !dynamic_call {
                    let function_parameters = info.parameters.iter().map(|ty| ty.resolve(type_index, context)).collect();
                    let parameters = FunctionInstanceParameters {
                        function_blueprint,
                        this_type,
                        function_parameters,
                    };

                    let function_instance = context.get_function_instance(parameters);

                    content.extend(info.args.resolve(type_index, context));
                    content.extend_from_slice(&function_instance.wasm_call);
                }


                content
            },
            VirtualInstruction::Loop(info) => vec![wat!["loop", info.content.resolve(type_index, context)]],
            VirtualInstruction::Block(info) => {
                let mut wat = wat!["block"];

                if !info.result.is_empty() {
                    let mut result = wat!["result"];

                    for ty in &info.result {
                        if let Some(wasm_type) = ty.resolve(type_index, context).wasm_type {
                            result.push(wasm_type);
                        }
                    }

                    wat.push(result);
                }

                wat.extend(info.content.resolve(type_index, context));

                vec![wat]
            },
            VirtualInstruction::Jump(info) => vec![wat!["br", info.depth]],
            VirtualInstruction::JumpIf(info) => {
                let mut jump = wat!["br_if", info.depth];

                if let Some(vasm) = &info.condition {
                    jump.extend(vasm.resolve(type_index, context));
                }

                vec![jump]
            },
            VirtualInstruction::IfThenElse(info) => {
                let condition_wasm = info.condition.resolve(type_index, context);
                let then_branch = info.then_branch.resolve(type_index, context);
                let else_branch = info.else_branch.resolve(type_index, context);

                let mut content = vec![];
                let mut if_block = wat!["if"];

                if let Some(return_type) = &info.return_type {
                    if let Some(wasm_type) = return_type.resolve(type_index, context).wasm_type {
                        if_block.push(wat!["result", wasm_type]);
                    }
                }

                if_block.push(wat!["then", then_branch]);
                if_block.push(wat!["else", else_branch]);

                content.extend(condition_wasm);
                content.push(if_block);

                content
            },
        }
    }

    pub fn resolve_without_context(&self) -> Vec<Wat> {
        match self {
            VirtualInstruction::Drop => vec![wat!["drop"]],
            VirtualInstruction::Eqz => vec![wat!["i32.eqz"]],
            VirtualInstruction::Raw(wat) => vec![wat.to_owned()],
            VirtualInstruction::IntConstant(_) => unreachable!(),
            VirtualInstruction::FloatConstant(_) => unreachable!(),
            VirtualInstruction::TypeId(_) => unreachable!(),
            VirtualInstruction::TypeName(_) => unreachable!(),
            VirtualInstruction::Store(_) => unreachable!(),
            VirtualInstruction::Load(_) => unreachable!(),
            VirtualInstruction::GetVariable(_) => unreachable!(),
            VirtualInstruction::SetVariable(_) => unreachable!(),
            VirtualInstruction::TeeVariable(_) => unreachable!(),
            VirtualInstruction::GetField(_) => unreachable!(),
            VirtualInstruction::SetField(_) => unreachable!(),
            VirtualInstruction::FunctionCall(_) => unreachable!(),
            VirtualInstruction::Loop(_) => unreachable!(),
            VirtualInstruction::Block(_) => unreachable!(),
            VirtualInstruction::Jump(_) => unreachable!(),
            VirtualInstruction::JumpIf(_) => unreachable!(),
            VirtualInstruction::IfThenElse(_) => unreachable!(),
        }
    }
}