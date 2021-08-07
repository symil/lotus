use parsable::parsable;

use crate::program::{AccessType, ProgramContext, Type, Wasm};

use super::{VarPathRoot, VarPathSegment};

#[parsable]
pub struct VarPath {
    pub root: VarPathRoot,
    pub path: Vec<VarPathSegment>
}

impl VarPath {
    pub fn process(&self, access_type: AccessType, context: &mut ProgramContext) -> Option<Wasm> {
        let mut current_access_type = match self.path.is_empty() {
            true => access_type,
            false => AccessType::Get
        };

        let mut parent_type = Type::Void;
        let mut wat = vec![];

        if let Some(root_wasm) = self.root.process(current_access_type, context) {
            parent_type = root_wasm.ty;
            wat.extend(root_wasm.wat);

            for (i, segment) in self.path.iter().enumerate() {
                if i == self.path.len() - 1 {
                    current_access_type = access_type;
                }

                if let Some(segment_wasm) = segment.process(&parent_type, current_access_type, context) {
                    parent_type = segment_wasm.ty;
                    wat.extend(segment_wasm.wat);
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }

        Some(Wasm::typed(parent_type, wat))
    }
}