use parsable::parsable;
use crate::{program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{Identifier, VarPathRoot, VarPathSegment};

#[parsable]
pub struct VarPath {
    pub root: Box<VarPathRoot>,
    pub path: Vec<VarPathSegment>
}

impl VarPath {
    pub fn collected_instancied_type_names(&self, list: &mut Vec<Identifier>) {
        self.root.collected_instancied_type_names(list);
    }

    pub fn process(&self, type_hint: Option<&Type>, access_type: AccessType, context: &mut ProgramContext) -> Option<Vasm> {
        let mut current_access_type = match self.path.is_empty() {
            true => access_type,
            false => AccessType::Get
        };

        let mut parent_type = Type::Undefined;
        let mut ok = true;
        let mut source = vec![];
        let mut current_type_hint = match self.path.is_empty() {
            true => type_hint,
            false => None,
        };

        if let Some(root_vasm) = self.root.process(current_type_hint, current_access_type, context) {
            parent_type = root_vasm.ty.clone();
            source.push(root_vasm);

            for (i, segment) in self.path.iter().enumerate() {
                if i == self.path.len() - 1 {
                    current_access_type = access_type;
                    current_type_hint = type_hint;
                }

                if let Some(segment_wasm) = segment.process(&parent_type, current_type_hint, current_access_type, context) {
                    parent_type = segment_wasm.ty.clone();
                    source.push(segment_wasm);
                } else {
                    ok = false;
                    break;
                }
            }
        } else {
            ok = false;
        }

        match ok {
            true => Some(Vasm::merge(source)),
            false => None
        }
    }
}