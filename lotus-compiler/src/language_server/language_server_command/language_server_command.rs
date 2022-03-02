use std::{mem::take, time::Instant, fmt::format};
use parsable::ParseError;
use crate::{command_line::{infer_root_directory, bundle_with_prelude, time_function}, program::{ProgramContext, ProgramContextOptions, CursorLocation}, utils::{FileSystemCache, PerfTimer}, items::ParsedSourceFile};
use super::{LanguageServerCommandKind, LanguageServerCommandParameters, LanguageServerCommandOutput, LanguageServerCommandReload};

pub const COMMAND_OUTPUT_ITEM_LINE_START : &'static str = "\n#?!#";
pub const COMMAND_SEPARATOR : &'static str = "##";

pub struct LanguageServerCommand {
    pub id: u32,
    pub kind: LanguageServerCommandKind,
    pub root_directory_path: String,
    pub file_path: String,
    pub cursor_index: usize,
    pub file_content: String,
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

        let duration = 0;
        let parameters = LanguageServerCommandParameters {
            new_name
        };

        Some(Self {
            id,
            kind,
            root_directory_path,
            file_path,
            cursor_index,
            file_content,
            parameters,
        })
    }

    pub fn run(mut self, mut cache: Option<&mut FileSystemCache<ParsedSourceFile, ParseError>>) -> String {
        let callback = self.kind.get_callback();
        let mut timer = PerfTimer::new();
        let mut context = ProgramContext::new(ProgramContextOptions::language_server(&self.root_directory_path, &self.file_path, self.cursor_index));
        let mut output = LanguageServerCommandOutput::new(self.id);

        if let Some(cache) = &mut cache {
            cache.delete_hook();

            if !self.file_content.is_empty() {
                cache.set_hook(&self.file_path, take(&mut self.file_content));
            }
        }

        timer.trigger("parsing");
        context.parse_source_files(&bundle_with_prelude(&self.root_directory_path), cache);

        timer.trigger("processing");
        if !context.has_errors() {
            context.process_source_files();
        }

        timer.trigger("cleanup");
        callback(&self.parameters, &context, &mut output);

        context.destroy();

        // let header = format!("{}ms", timer.get_total());
        let header = timer.to_string(", ", 0);

        output.format(Some(header))
    }
}