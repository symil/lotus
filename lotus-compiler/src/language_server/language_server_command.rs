use std::mem::take;
use parsable::ParseError;
use crate::{command_line::{infer_root_directory, bundle_with_prelude, time_function}, program::{ProgramContext, ProgramContextOptions, CursorInfo}, utils::FileSystemCache, items::LotusFile};
use super::{LanguageServerCommandKind, LanguageServerCommandParameters, LanguageServerCommandOutput, LanguageServerCommandReload};

pub const COMMAND_SEPARATOR : &'static str = "##";

pub struct LanguageServerCommand {
    pub id: u32,
    pub kind: LanguageServerCommandKind,
    pub parameters: LanguageServerCommandParameters
}

impl LanguageServerCommand {
    pub fn from_str(string: &str) -> Option<Self> {
        let mut arguments = string.split(COMMAND_SEPARATOR);
        let id = arguments.next().and_then(|str| str.parse::<u32>().ok()).unwrap_or(0);
        let kind = arguments.next().and_then(|str| LanguageServerCommandKind::from_str(str))?;
        let file_path = arguments.next().and_then(|str| Some(str.to_string()))?;
        let cursor_index = arguments.next().and_then(|str| str.parse::<usize>().ok()).unwrap_or(usize::MAX);
        let file_content = arguments.next().and_then(|str| Some(str.to_string())).unwrap_or_default();
        let new_name = arguments.next().and_then(|str| Some(str.to_string())).unwrap_or_default();
        let root_directory_path = infer_root_directory(&file_path).unwrap_or_default();

        let parameters = LanguageServerCommandParameters {
            root_directory_path,
            file_path,
            cursor_index,
            file_content,
            new_name
        };

        Some(Self {
            id,
            kind,
            parameters,
        })
    }

    pub fn run(mut self, mut cache: Option<&mut FileSystemCache<LotusFile, ParseError>>) -> LanguageServerCommandOutput {
        let callback = self.kind.get_callback();
        let mut context = ProgramContext::new(ProgramContextOptions {
            validate_only: true,
            cursor: Some(CursorInfo {
                file_path: self.parameters.file_path.to_string(),
                index: self.parameters.cursor_index,
            }),
        });
        let mut output = LanguageServerCommandOutput::new(self.id);

        if let Some(cache) = &mut cache {
            cache.delete_hook();

            if !self.parameters.file_content.is_empty() {
                cache.set_hook(&self.parameters.file_path, take(&mut self.parameters.file_content));
            }
        }

        context.parse_source_files(&bundle_with_prelude(&self.parameters.root_directory_path), cache);

        if !context.has_errors() {
            context.process_source_files();
        }

        callback(&self.parameters, &context, &mut output);

        output
    }
}