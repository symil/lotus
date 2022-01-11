use crate::{program::{Type, InterfaceBlueprint, VariableInfo, GlobalVarBlueprint, TypeBlueprint, TypedefBlueprint, FunctionBlueprint}, utils::Link};

#[derive(Debug)]
pub enum CompletionContent {
    FieldOrMethod(FieldCompletionDetails),
    StaticField(FieldCompletionDetails),
    Event(EventCompletionDetails),
    Interface(Vec<Link<InterfaceBlueprint>>),
    Type(TypeCompletionDetails),
    Variable(VariableCompletionDetails)
}

#[derive(Debug)]
pub struct EventCompletionDetails {
    pub available_events: Vec<Type>,
    pub insert_brackets: bool
}

#[derive(Debug)]
pub struct FieldCompletionDetails {
    pub parent_type: Type,
    pub insert_arguments: bool
}

#[derive(Debug)]
pub struct TypeCompletionDetails {
    pub available_types: Vec<Type>,
    pub self_type: Option<Type>
}

#[derive(Debug)]
pub struct VariableCompletionDetails {
    pub available_variables: Vec<VariableInfo>,
    pub available_globals: Vec<Link<GlobalVarBlueprint>>,
    pub available_functions: Vec<Link<FunctionBlueprint>>,
    pub available_types: Vec<Link<TypeBlueprint>>,
    pub available_typedefs: Vec<Link<TypedefBlueprint>>,
    pub self_type: Option<Link<TypeBlueprint>>,
    pub insert_arguments: bool,
}