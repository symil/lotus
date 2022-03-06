use parsable::{ItemLocation, parsable};
use crate::{program::{ProgramContext, Vasm, FieldKind, Type, AccessType}, language_server::FieldCompletionOptions};
use super::{ParsedVarPrefix, Identifier, ParsedArgumentList, process_method_call, process_field_access, unwrap_item};

#[parsable]
pub struct ParsedPrefixedVarRef {
    pub prefix: ParsedVarPrefix,
    pub name: Option<Identifier>,
    pub arguments: Option<ParsedArgumentList>
}

impl ParsedPrefixedVarRef {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn collect_instancied_type_names(&self, list: &mut Vec<String>, context: &mut ProgramContext) {
        list.push("system".to_string());
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut vasm = self.prefix.process(context);

        context.completion_provider.add_field_completion(&self.prefix, &vasm.ty, Some(&FieldCompletionOptions {
            show_methods: true,
            insert_arguments: self.arguments.is_none(),
            ..Default::default()
        }));

        let name = unwrap_item(&self.name, &self.prefix, context)?;
        
        context.completion_provider.add_field_completion(name, &vasm.ty, Some(&FieldCompletionOptions {
            show_methods: true,
            insert_arguments: self.arguments.is_none(),
            ..Default::default()
        }));

        match &self.arguments {
            Some(args) => match process_method_call(&vasm.ty, FieldKind::Regular, name, &[], args, type_hint, access_type, context) {
                Some(method_vasm) => Some(vasm.append(method_vasm)),
                None => None,
            },
            None => match process_field_access(&vasm.ty, FieldKind::Regular, &name, access_type, context) {
                Some(field_vasm) => Some(vasm.append(field_vasm)),
                None => None,
            },
        }
    }
}