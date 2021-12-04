use std::{fs::{self, DirBuilder, File}, io::Write, path::{Path, PathBuf}, time::Instant};
use parsable::*;
use crate::{items::LotusFile, program::{CompilationError, ProgramContext}};

use super::{Timer, ProgramStep};

const SOURCE_FILE_EXTENSION : &'static str = "lt";
const COMMENT_START_TOKEN : &'static str = "//";
const PRELUDE_NAMESPACE : &'static str = "std";
const SELF_NAMESPACE : &'static str = "self";

pub struct LotusProgram {
    pub wat: String,
}

impl LotusProgram {
    pub fn from_path(path: &str, prelude_directory_path: Option<&str>, timer: &mut Timer) -> Result<Self, Vec<CompilationError>> {
        timer.start(ProgramStep::Read);

        let mut parsed_files = vec![];
        let mut errors = vec![];

        let mut input_path = Path::new(path).to_path_buf();
        let prefix = match input_path.is_dir() {
            true => path,
            false => {
                input_path.pop();
                input_path.to_str().unwrap()
            },
        };

        let mut source_list = vec![];

        if let Some(prelude_path) = prelude_directory_path {
            source_list.push((PRELUDE_NAMESPACE, prelude_path, prelude_path));
        }

        source_list.push((SELF_NAMESPACE, path, prefix));

        let mut source_files = vec![];

        for (file_namespace, path, prefix) in &source_list {
            let mut files_to_process = read_path_recursively(path, true)?;

            files_to_process.sort_by_cached_key(|(path, content)| path.to_str().unwrap().to_string());

            for (file_path, file_content) in files_to_process {
                let file_name = file_path.strip_prefix(prefix).unwrap().to_str().unwrap().to_string();

                source_files.push((*file_namespace, file_name, file_content));
            }
        }

        timer.stop(ProgramStep::Read);

        timer.start(ProgramStep::Parse);

        for (file_namespace, file_name, file_content) in source_files {
            let parse_options = ParseOptions {
                file_name: Some(&file_name),
                file_namespace: Some(file_namespace),
                comment_start: Some(COMMENT_START_TOKEN),
            };

            if !file_content.starts_with("// ignore") {
                match LotusFile::parse(file_content, parse_options) {
                    Ok(lotus_file) => parsed_files.push(lotus_file),
                    Err(parse_error) => errors.push(CompilationError::parse_error(parse_error))
                };
            }
        }

        timer.stop(ProgramStep::Parse);

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut context = ProgramContext::new();

        context.process_files(parsed_files, timer);

        let wat = context.generate_wat(timer)?;

        Ok(Self { wat })
    }

    pub fn write_to(&self, output_file_path: &str) {
        let path = Path::new(output_file_path);

        if let Some(parent_dir) = path.to_path_buf().parent() {
            DirBuilder::new().recursive(true).create(parent_dir).unwrap();
        }

        let mut file = File::create(path).unwrap();

        file.write_all(self.wat.as_bytes()).unwrap();
    }
}

fn read_path_recursively(input_path: &str, is_first: bool) -> Result<Vec<(PathBuf, String)>, Vec<CompilationError>> {
    let mut result = vec![];
    let path = Path::new(input_path);

    if path.to_str().unwrap().contains(".ignore") {
        return Ok(vec![]);
    }

    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension == SOURCE_FILE_EXTENSION {
                let file_content = match fs::read_to_string(&path) {
                    Ok(content) => content,
                    Err(_) => return Err(vec![CompilationError::generic_unlocated(format!("cannot read file `{}`", input_path))])
                };

                result.push((path.to_path_buf(), file_content))
            }
        }

        if is_first && result.is_empty() {
            return Err(vec![CompilationError::generic_unlocated(format!("specified source file must have a `.{}` extension", SOURCE_FILE_EXTENSION))]);
        }
    } else if path.is_dir() {
        let entries = match path.read_dir() {
            Ok(content) => content,
            Err(_) => return Err(vec![CompilationError::generic_unlocated(format!("cannot read directory `{}`", input_path))]),
        };

        for entry in entries {
            if let Ok(entry) = entry {
                result.append(&mut read_path_recursively(entry.path().to_str().unwrap(), false)?);
            }
        }
    } else if is_first {
        return Err(vec![CompilationError::generic_unlocated(format!("path `{}` is not a valid file or directory", input_path))]);
    }

    Ok(result)
}