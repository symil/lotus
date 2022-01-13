use std::{rc::Rc, ops::Deref, borrow::Borrow};
use parsable::DataLocation;
use crate::utils::Link;
use super::{ProgramContext, Type, TypeIndex, VariableInfo, VirtualInstruction, Wat, VirtualInitVariableInfo, ToInt, PlaceholderDetails, VariableAccessKind, VirtualVariableAccessInfo, FunctionBlueprint, VirtualFunctionIndexInfo, VirtualAccessFieldInfo, FieldAccessKind, IfThenElseInfo, VirtualJumpIfInfo, VirtualBlockInfo, VirtualJumpInfo, VirtualLoopInfo, FunctionCall, NamedFunctionCallDetails, VirtualFunctionCallInfo, NONE_METHOD_NAME, AnonymousFunctionCallDetails, Signature};

pub type Vasm = VirtualAssembly;

#[derive(Debug, Clone)]
pub struct VirtualAssembly {
    pub ty: Type,
    content: Option<VirtualAssemblyContent>
}

#[derive(Debug, Clone)]
struct VirtualAssemblyContent {
    variables: Vec<VariableInfo>,
    instructions: Vec<VirtualInstruction>
}

impl VirtualAssembly {
    pub fn new(allow_populating: bool) -> Self {
        let ty = Type::undefined();
        let content = match allow_populating {
            true => Some(VirtualAssemblyContent {
                variables: vec![],
                instructions: vec![],
            }),
            false => None,
        };

        Self { ty, content }
    }

    pub fn undefined() -> Self {
        Self::new(false)
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            Some(content) => content.instructions.is_empty(),
            None => unreachable!(),
        }
    }

    pub fn append(mut self, other: Self) -> Self {
        self.ty = other.ty;

        if let (Some(self_content), Some(other_content)) = (&mut self.content.as_mut(), other.content) {
            self_content.variables.extend(other_content.variables);
            self_content.instructions.extend(other_content.instructions);
        }

        self
    }

    pub fn chain<F : FnOnce(Self) -> Self>(self, callback: F) -> Self {
        callback(self)
    }

    pub fn set_type<T : Borrow<Type>>(mut self, ty: T) -> Self {
        self.ty = ty.borrow().clone();
        self
    }

    pub fn set_void(self, context: &ProgramContext) -> Self {
        let ty = self.ty.clone();
        let mut result = match ty.is_undefined() {
            true => self,
            false => self.drop(&ty),
        };

        result.set_type(context.void_type())
    }

    pub fn declare_variable<T : Borrow<VariableInfo>>(mut self, var_info: T) -> Self {
        if let Some(content) = self.content.as_mut() {
            content.variables.push(var_info.borrow().clone());
        }
        self
    }

    fn instruction<F : FnOnce() -> VirtualInstruction>(mut self, callback: F) -> Self {
        if let Some(content) = self.content.as_mut() {
            content.instructions.push(callback());
        }
        self
    }

    pub fn eqz(self) -> Self {
        self.instruction(|| VirtualInstruction::Eqz)
    }

    pub fn raw(self, value: Wat) -> Self {
        self.instruction(|| VirtualInstruction::Raw(value))
    }
    
    pub fn drop(self, ty: &Type) -> Self {
        self.instruction(|| VirtualInstruction::Drop(ty.clone()))
    }
    
    pub fn placeholder(self, location: &DataLocation) -> Self {
        self.instruction(|| VirtualInstruction::Placeholder(PlaceholderDetails {
            location: location.clone(),
            vasm: None,
        }))
    }

    pub fn return_value(self, value: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::Return(value))
    }

    pub fn int<T : ToInt>(self, value: T) -> Self {
        self.instruction(|| VirtualInstruction::IntConstant(value.to_i32()))
    }

    pub fn float(self, value: f32) -> Self {
        self.instruction(|| VirtualInstruction::FloatConstant(value))
    }

    pub fn type_id(self, ty: &Type) -> Self {
        self.instruction(|| VirtualInstruction::TypeId(ty.clone()))
    }

    pub fn type_name(self, ty: &Type) -> Self {
        self.instruction(|| VirtualInstruction::TypeName(ty.clone()))
    }

    pub fn init_var(self, var_info: &VariableInfo) -> Self {
        self.instruction(|| VirtualInstruction::InitVariable(VirtualInitVariableInfo {
            var_info: var_info.clone(),
        }))
    }

    pub fn get_var(self, var_info: &VariableInfo, access_level: Option<u32>) -> Self {
        self.instruction(|| VirtualInstruction::VariableAccess(VirtualVariableAccessInfo{
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Get,
            access_level,
            value: None
        }))
    }

    pub fn get_tmp_var(self, var_info: &VariableInfo) -> Self {
        self.get_var(var_info, None)
    }

    pub fn set_var(self, var_info: &VariableInfo, access_level: Option<u32>, value: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::VariableAccess(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Set,
            access_level,
            value: Some(value),
        }))
    }

    pub fn set_tmp_var(self, var_info: &VariableInfo) -> Self {
        self.set_var(var_info, None, Vasm::undefined())
    }

    pub fn tee_var(self, var_info: &VariableInfo, access_level: Option<u32>, value: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::VariableAccess(VirtualVariableAccessInfo {
            var_info: var_info.clone(),
            access_kind: VariableAccessKind::Tee,
            access_level,
            value: Some(value),
        }))
    }

    pub fn tee_tmp_var(self, var_info: &VariableInfo) -> Self {
        self.tee_var(var_info, None, Vasm::undefined())
    }

    pub fn call_function_named(self, caller_type: Option<&Type>, function: &Link<FunctionBlueprint>, parameters: &[Type], arguments: Vec<Vasm>) -> Self {
        self.instruction(|| {
            VirtualInstruction::FunctionCall(VirtualFunctionCallInfo {
                call: FunctionCall::Named(NamedFunctionCallDetails {
                    caller_type: caller_type.cloned(),
                    function: function.clone(),
                    parameters: parameters.to_vec(),
                }),
                function_index_var: None,
                arguments,
            })
        })
    }

    pub fn call_function_anonymous(self, signature: &Signature, function_offset: usize, arguments: Vec<Vasm>, context: &ProgramContext) -> Self {
        self.instruction(|| {
            VirtualInstruction::FunctionCall(VirtualFunctionCallInfo {
                call: FunctionCall::Anonymous(AnonymousFunctionCallDetails {
                    signature: signature.clone(),
                    function_offset,
                }),
                function_index_var: Some(VariableInfo::tmp("function_index", context.int_type())),
                arguments,
            })
        })
    }

    pub fn call_regular_method(self, caller_type: &Type, method_name: &str, parameters: &[Type], arguments: Vec<Vasm>, context: &ProgramContext) -> Self {
        if caller_type.is_undefined() {
            return self;
        }

        let method_blueprint = match caller_type.get_regular_method(method_name, context) {
            Some(result) => result.function,
            None => panic!("type {} has no regular method `{}`", caller_type.to_string(), method_name)
        };

        self.call_function_named(
            Some(caller_type),
            &method_blueprint,
            parameters,
            arguments
        )
    }

    pub fn call_static_method(self, caller_type: &Type, method_name: &str, parameters: &[Type], arguments: Vec<Vasm>, context: &ProgramContext) -> Self {
        if caller_type.is_undefined() {
            return self;
        }
        
        let method_blueprint = match caller_type.get_static_method(method_name, context) {
            Some(result) => result.function,
            None => panic!("type {} has no static method `{}`", caller_type.to_string(), method_name)
        };

        self.call_function_named(
            Some(caller_type),
            &method_blueprint,
            parameters,
            arguments
        )
    }

    pub fn none(self, ty: &Type, context: &ProgramContext) -> Self {
        self.call_static_method(ty, NONE_METHOD_NAME, &[], vec![], context)
    }

    pub fn function_index(self, function: &Link<FunctionBlueprint>, parameters: &[Type]) -> Self {
        self.instruction(|| VirtualInstruction::FunctionIndex(VirtualFunctionIndexInfo {
            function: function.clone(),
            parameters: parameters.to_vec(),
        }))
    }

    pub fn get_field(self, field_type: &Type, field_offset: usize) -> Self {
        self.instruction(|| VirtualInstruction::FieldAccess(VirtualAccessFieldInfo {
            acess_kind: FieldAccessKind::Get,
            field_type: field_type.clone(),
            field_offset,
            value: None,
        }))
    }

    pub fn set_field(self, field_type: &Type, field_offset: usize, value: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::FieldAccess(VirtualAccessFieldInfo {
            acess_kind: FieldAccessKind::Set,
            field_type: field_type.clone(),
            field_offset,
            value: Some(value),
        }))
    }

    pub fn loop_(self, content: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::Loop(VirtualLoopInfo {
            content: content,
        }))
    }

    pub fn block(self, content: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::Block(VirtualBlockInfo {
            result: vec![],
            content: content,
        }))
    }

    pub fn typed_block(self, result: Vec<Type>, content: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::Block(VirtualBlockInfo {
            result,
            content,
        }))
    }

    pub fn jump(self, depth: u32) -> Self {
        self.instruction(|| VirtualInstruction::Jump(VirtualJumpInfo {
            depth
        }))
    }

    pub fn jump_if(self, depth: u32, condition: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: Some(condition),
        }))
    }

    pub fn jump_if_from_stack(self, depth: u32) -> Self {
        self.instruction(|| VirtualInstruction::JumpIf(VirtualJumpIfInfo {
            depth,
            condition: None
        }))
    }

    pub fn if_then_else(self, return_type: Option<&Type>, condition: Vasm, then_branch: Vasm, else_branch: Vasm) -> Self {
        self.instruction(|| VirtualInstruction::IfThenElse(IfThenElseInfo {
            return_type: return_type.cloned(),
            condition,
            then_branch,
            else_branch,
        }))
    }

    pub fn collect_variables(&self, list: &mut Vec<VariableInfo>) {
        if let Some(content) = &self.content {
            list.extend(content.variables.clone());

            for instruction in &content.instructions {
                instruction.collect_variables(list);
            }
        }
    }

    pub fn replace_placeholder(&mut self, location: &DataLocation, replacement: &Rc<Vasm>) {
        if let Some(content) = &mut self.content {
            for instruction in &mut content.instructions {
                instruction.replace_placeholder(location, replacement);
            }
        }
    }

    pub fn resolve(&self, type_index: &TypeIndex, context: &mut ProgramContext) -> Vec<Wat> {
        let mut result = vec![];

        if let Some(content) = &self.content {
            for inst in &content.instructions {
                result.extend(inst.resolve(type_index, context));
            }
        }

        result
    }

    pub fn resolve_without_context(&self) -> Vec<Wat> {
        let mut result = vec![];

        if let Some(content) = &self.content {
            for inst in &content.instructions {
                result.extend(inst.resolve_without_context());
            }
        }

        result
    }
}