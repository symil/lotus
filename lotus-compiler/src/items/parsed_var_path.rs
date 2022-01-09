use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{Identifier, ParsedVarPathRoot, ParsedVarPathSegment};

#[parsable]
pub struct ParsedVarPath {
    pub root: Box<ParsedVarPathRoot>,
    pub path: Vec<ParsedVarPathSegment>
}

impl ParsedVarPath {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>, context: &mut ProgramContext) {
        self.root.collected_instancied_type_names(list, context);
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut current_access_type = match self.path.is_empty() {
            true => access_type,
            false => AccessType::Get
        };

        let mut parent_type = Type::undefined();
        let mut result = context.vasm();
        let mut current_type_hint = match self.path.is_empty() {
            true => type_hint,
            false => None,
        };

        if let Some(root_vasm) = self.root.process(current_type_hint, current_access_type, context) {
            parent_type = root_vasm.ty.clone();
            result = result.append(root_vasm);

            for (i, segment) in self.path.iter().enumerate() {
                if i == self.path.len() - 1 {
                    current_access_type = access_type;
                    current_type_hint = type_hint;
                }

                if let Some(segment_vasm) = segment.process(&parent_type, current_type_hint, current_access_type, context) {
                    parent_type = segment_vasm.ty.clone();
                    result = result.append(segment_vasm);
                } else {
                    return None;
                }
            }
        }

        Some(result)
    }
}