use std::{fs, path::{PathBuf}};
use parsable::*;
use crate::items::LotusFile;
use super::{error::Error, program_index::ProgramIndex};

pub struct LotusProgram {
    pub index: ProgramIndex
}

impl LotusProgram {
    pub fn from_directory_path(directory_path: &'static str) -> Result<Self, Vec<Error>> {
        let file_paths = read_directory(directory_path);
        let mut parsed_files = vec![];
        let mut string_reader = StringReader::new();
        let mut errors = vec![];

        for path in file_paths {
            let file_content = fs::read_to_string(&path).expect(&format!("cannot read file {:?}", &path));
            let file_name = path.strip_prefix(directory_path).unwrap().to_str().unwrap().to_string();

            string_reader.set_content(file_content, file_name);

            match LotusFile::parse_string(&mut string_reader) {
                Ok(lotus_file) => parsed_files.push(lotus_file),
                Err(parse_error) => errors.push(Error::from_parse_error(parse_error, string_reader.get_file_name()))
            };
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let index = ProgramIndex::from_parsed_files(parsed_files)?;

        Ok(LotusProgram { index })
    }
}

fn read_directory(directory_path: &str) -> Vec<PathBuf> {
    let entries = fs::read_dir(directory_path).expect(&format!("cannot read directory {}", directory_path));
    let mut result = vec![];

    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                let path = entry.path();

                if metadata.is_dir() {
                    result.append(&mut read_directory(path.to_str().unwrap()));
                } else if metadata.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "lt" {
                            result.push(entry.path());
                        }
                    }
                }
            }
        }
    }

    result
}