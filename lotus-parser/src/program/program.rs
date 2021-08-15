use std::{fs::{self, DirBuilder, File}, io::Write, path::{Path, PathBuf}, time::Instant};
use parsable::*;
use crate::{items::LotusFile, program::ProgramContext};
use super::Error;

const SOURCE_FILE_EXTENSION : &'static str = "lt";
const COMMENT_START_TOKEN : &'static str = "//";

pub struct LotusProgram {
    pub wat: String,
    pub process_time: f64
}

impl LotusProgram {
    pub fn from_path(path: &str, prelude: &'static[(&'static str, &'static str)]) -> Result<Self, Vec<Error>> {
        let files_to_process = read_path_recursively(path, true)?;
        let mut parsed_files = vec![];
        let mut string_reader = StringReader::new(COMMENT_START_TOKEN);
        let mut errors = vec![];

        let mut input_path = Path::new(path).to_path_buf();
        let prefix = match input_path.is_dir() {
            true => path,
            false => {
                input_path.pop();
                input_path.to_str().unwrap()
            },
        };

        let now = Instant::now();

        for (file_name, file_content) in prelude {
            string_reader.set_content(file_content.to_string(), file_name.to_string());

            match LotusFile::parse_string(&mut string_reader) {
                Ok(lotus_file) => parsed_files.push(lotus_file),
                Err(parse_error) => errors.push(Error::from_parse_error(parse_error, string_reader.get_file_name()))
            };
        }

        for (file_path, file_content) in files_to_process {
            let file_name = file_path.strip_prefix(prefix).unwrap().to_str().unwrap().to_string();

            string_reader.set_content(file_content, file_name);

            match LotusFile::parse_string(&mut string_reader) {
                Ok(lotus_file) => parsed_files.push(lotus_file),
                Err(parse_error) => errors.push(Error::from_parse_error(parse_error, string_reader.get_file_name()))
            };
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut context = ProgramContext::new();

        context.process_files(parsed_files);

        let process_time = now.elapsed().as_secs_f64();
        let wat = context.generate_wat()?;

        Ok(Self { wat, process_time })
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

fn read_path_recursively(input_path: &str, is_first: bool) -> Result<Vec<(PathBuf, String)>, Vec<Error>> {
    let mut result = vec![];
    let path = Path::new(input_path);

    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension == SOURCE_FILE_EXTENSION {
                let file_content = match fs::read_to_string(&path) {
                    Ok(content) => content,
                    Err(_) => return Err(vec![Error::unlocated(format!("cannot read file `{}`", input_path))])
                };

                result.push((path.to_path_buf(), file_content))
            }
        }

        if is_first && result.is_empty() {
            return Err(vec![Error::unlocated(format!("specified source file must have a `.{}` extension", SOURCE_FILE_EXTENSION))]);
        }
    } else if path.is_dir() {
        let entries = match path.read_dir() {
            Ok(content) => content,
            Err(_) => return Err(vec![Error::unlocated(format!("cannot read directory `{}`", input_path))]),
        };

        for entry in entries {
            if let Ok(entry) = entry {
                result.append(&mut read_path_recursively(entry.path().to_str().unwrap(), false)?);
            }
        }
    } else if is_first {
        return Err(vec![Error::unlocated(format!("path `{}` is not a valid file or directory", input_path))]);
    }

    Ok(result)
}