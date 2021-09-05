use parsable::parsable;

use crate::program::{AccessType, ProgramContext, TypeOld, Wasm};

use super::{VarPathRoot, VarPathSegment};

#[parsable]
pub struct VarPath {
    pub root: VarPathRoot,
    pub path: Vec<VarPathSegment>
}

impl VarPath {
    pub fn has_side_effects(&self) -> bool {
        match self.root.has_side_effects() {
            true => true,
            false => self.path.iter().any(|segment| segment.has_side_effects()),
        }
    }

    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        let mut current_access_type = match self.path.is_empty() {
            true => access_type,
            false => AccessType::Get
        };

        let mut parent_type = TypeOld::Void;
        let mut ok = true;
        let mut source = vec![];

        if let Some(root_wasm) = self.root.process(current_access_type, context) {
            parent_type = root_wasm.ty.clone();
            source.push(root_wasm);

            for (i, segment) in self.path.iter().enumerate() {
                if i == self.path.len() - 1 {
                    current_access_type = access_type;
                }

                if let Some(segment_wasm) = segment.process(&parent_type, current_access_type, context) {
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
            true => Some(Wasm::merge(parent_type, source)),
            false => None
        }
    }
}