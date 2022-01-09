use parsable::{DataLocation, parsable};
use crate::program::{ProgramContext, Vasm, FieldKind, Type, AccessType};
use super::{VarPrefix, Identifier, ArgumentList, process_method_call, process_field_access};

#[parsable]
pub struct PrefixedVarRef {
    pub prefix: VarPrefix,
    pub name: Option<Identifier>,
    pub arguments: Option<ArgumentList>
}

impl PrefixedVarRef {
    pub fn has_side_effects(&self) -> bool {
        self.arguments.is_some()
    }

    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {

    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut vasm = self.prefix.process(context);

        context.add_field_completion_area(&self.prefix, &vasm.ty, self.arguments.is_none());

        let name = match &self.name {
            Some(name) => name,
            None => {
                return context.errors.expected_identifier(&self.prefix).none();
            },
        };
        
        context.add_field_completion_area(name, &vasm.ty, self.arguments.is_none());

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