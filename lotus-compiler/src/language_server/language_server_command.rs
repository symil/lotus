use parsable::ParseError;

use crate::{command_line::{infer_root_directory, bundle_with_prelude}, program::{ProgramContext, ProgramContextOptions, CursorInfo}, utils::FileSystemCache, items::LotusFile};
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
        let payload = arguments.next().and_then(|str| Some(str.to_string()));
        let root_directory_path = infer_root_directory(&file_path).unwrap_or_default();

        let parameters = LanguageServerCommandParameters {
            root_directory_path,
            file_path,
            cursor_index,
            payload
        };

        Some(Self {
            id,
            kind,
            parameters,
        })
    }

    pub fn run(mut self, context: &mut ProgramContext, mut cache: Option<&mut FileSystemCache<LotusFile, ParseError>>, force_reset: bool) -> LanguageServerCommandOutput {
        let mut output = LanguageServerCommandOutput::new(self.id);
        let callback_details = self.kind.get_callback_details();
        let reset = force_reset || callback_details.reload != LanguageServerCommandReload::No;

        if reset {
            if let Some(cache) = &mut cache {
                cache.delete_hook();

                if callback_details.reload == LanguageServerCommandReload::WithHook {
                    if let Some(payload) = self.parameters.payload.take() {
                        cache.set_hook(&self.parameters.file_path, payload);
                    }
                }
            }

            *context = ProgramContext::new(ProgramContextOptions {
                validate_only: true,
                cursor: Some(CursorInfo::new(&self.parameters.file_path, self.parameters.cursor_index)),
            });
            context.parse_source_files(&bundle_with_prelude(&self.parameters.root_directory_path), cache);

            if !context.has_errors() {
                context.process_source_files();
            }
        }

        (callback_details.callback)(&self.parameters, &context, &mut output);

        output
    }
}