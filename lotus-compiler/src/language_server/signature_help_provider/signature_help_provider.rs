use indexmap::IndexMap;
use parsable::DataLocation;
use crate::{program::{CursorLocation, Signature, FunctionBlueprint, FunctionCall, Cursor}, utils::Link};
use super::SignatureHelp;

pub struct SignatureHelpProvider {
    cursor: Cursor,
    signature_helps: IndexMap<DataLocation, SignatureHelp>
}

impl SignatureHelpProvider {
    pub fn new(cursor: &Cursor) -> Self {
        Self {
            cursor: cursor.clone(),
            signature_helps: IndexMap::new()
        }
    }

    pub fn declare_signature(&mut self, location: &DataLocation, name: &str, function_call: &FunctionCall) {
        if !self.cursor.is_on_location(location) {
            return;
        }

        self.signature_helps
            .entry(location.clone())
            .or_insert_with(|| SignatureHelp::new(location, name, function_call));
    }

    pub fn add_argument_location(&mut self, signature_location: &DataLocation, argument_index: usize, next_arg_location: Option<&DataLocation>) {
        if !self.cursor.is_on_location(signature_location) {
            return;
        }

        let area = match self.signature_helps.get_mut(signature_location) {
            Some(area) => area,
            None => {
                panic!("cannot find signature help");
            },
        };

        area.set_argument_location(argument_index, next_arg_location, &self.cursor);
    }

    pub fn get_signature_help(&self) -> Option<&SignatureHelp> {
        for signature_help in self.signature_helps.values().rev() {
            if self.cursor.is_on_location(&signature_help.location) {
                return Some(signature_help);
            }
        }

        None
    }
}