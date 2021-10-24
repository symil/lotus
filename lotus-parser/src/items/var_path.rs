use parsable::parsable;
use crate::{items::ValueOrType, program::{AccessType, FieldKind, ProgramContext, Type, Vasm}};
use super::{Identifier, VarPathRoot, VarPathSegment};

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

        if let Some(value_or_type) = self.root.process(current_type_hint, current_access_type, context) {
            let (mut parent_type, mut field_kind) = match value_or_type {
                ValueOrType::Value(root_vasm) => {
                    let ty = root_vasm.ty.clone();
                    source.push(root_vasm);

                    (ty, FieldKind::Regular)
                },
                ValueOrType::Type(ty) => (ty, FieldKind::Static),
            };

            for (i, segment) in self.path.iter().enumerate() {
                if i == self.path.len() - 1 {
                    current_access_type = access_type;
                    current_type_hint = type_hint;
                }

                if let Some(segment_wasm) = segment.process(&parent_type, field_kind, current_type_hint, current_access_type, context) {
                    parent_type = segment_wasm.ty.clone();
                    field_kind = FieldKind::Regular;
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