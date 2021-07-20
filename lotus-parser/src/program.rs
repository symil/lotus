use std::{fs, path::{PathBuf}};

use crate::{items::{file::LotusFile}};
use parsable::*;

#[derive(Debug)]
pub struct LotusProgram {
    pub files: Vec<LotusFile>
}

impl LotusProgram {
    pub fn from_directory(directory_path: &'static str) -> Result<Self, ParseError> {
        let file_paths = read_directory(directory_path);
        let mut parsed_files = vec![];
        let mut string_reader = StringReader::new();

        for path in file_paths {
            let file_content = fs::read_to_string(&path).expect(&format!("cannot read file {:?}", &path));
            let file_name = path.strip_prefix(directory_path).unwrap().to_str().unwrap().to_string();

            string_reader.set_content(file_content, file_name);

            let lotus_file = LotusFile::parse_string(&mut string_reader)?;

            parsed_files.push(lotus_file);
        }

        Ok(LotusProgram {
            files: parsed_files
        })
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