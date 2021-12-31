use std::collections::HashSet;
use colored::Colorize;
use parsable::{DataLocation, ParseError};
use crate::utils::Link;
use super::{InterfaceBlueprint, Type};

#[derive(Debug)]
pub struct CompilationError {
    pub location: DataLocation,
    pub details: CompilationErrorDetails,
}

#[derive(Debug)]
pub enum CompilationErrorDetails {
    Generic(GenericErrorDetails),
    ParseError(ParseErrorDetails),
    TypeMismatch(TypeMismatchDetails),
    InterfaceMismatch(InterfaceMismatchDetails),
    UnexpectedKeyword(UnexpectedKeywordDetails),
    ExpectedExpression,
    UnexpectedExpression,
    UnexpectedVoidExpression,
    InvalidCharacter(InvalidCharacterDetails),
    ExpectedClassType(ExpectedClassTypeDetails)
}

#[derive(Debug)]
pub struct ExpectedClassTypeDetails {
    pub actual_type: Type
}

#[derive(Debug)]
pub struct GenericErrorDetails {
    pub error: String
}

#[derive(Debug)]
pub struct ParseErrorDetails {
    pub expected_tokens: Vec<String>
}

#[derive(Debug)]
pub struct TypeMismatchDetails {
    pub expected_type: Type,
    pub actual_type: Type
}

#[derive(Debug)]
pub struct InterfaceMismatchDetails {
    pub expected_interface: Link<InterfaceBlueprint>,
    pub actual_type: Type
}

#[derive(Debug)]
pub struct UnexpectedKeywordDetails {
    pub keyword: String
}

#[derive(Debug)]
pub struct InvalidCharacterDetails {
    pub character: String
}

impl CompilationError {
    pub fn to_string(&self) -> Option<String> {
        match self.get_message() {
            Some(first_line) => {
                let error_string = format!("{} {}", "error:".red().bold(), first_line);
                let (line, col) = self.location.get_line_col();
                let file_name = match self.location.file_path.starts_with(self.location.package_root_path) {
                    true => &self.location.file_path[(self.location.package_root_path.len() + 1)..],
                    false => self.location.file_path,
                };

                let location_string = format!("{}:{}:{}: ", file_name, line, col);
                let mut result = format!("{}{}", location_string.bold(), error_string);

                // for detail in self.get_details() {
                //     result.push_str(&format!("\n  - {}", detail));
                // }

                Some(result)
            },
            None => None,
        }
    }

    pub fn get_message(&self) -> Option<String> {
        match &self.details {
            CompilationErrorDetails::Generic(details) => {
                Some(details.error.clone())
            },
            CompilationErrorDetails::ParseError(detais) => {
                let mut expected_set = HashSet::new();
                let mut expected_list = vec![];

                for token in &detais.expected_tokens {
                    if expected_set.insert(token.clone()) {
                        expected_list.push(token.to_string());
                    }
                }

                Some(match expected_list.len() {
                    1 => format!("expected {}", expected_list[0]),
                    _ => format!("expected ({})", expected_list.join(" | ")),
                })
            },
            CompilationErrorDetails::TypeMismatch(details) => {
                let expected_str = details.expected_type.to_string();
                let actual_str = details.actual_type.to_string();

                match details.actual_type.is_undefined() {
                    true => None,
                    false => Some(format!("expected `{}`, got `{}`", expected_str, actual_str))
                }
            }
            CompilationErrorDetails::InterfaceMismatch(details) => {
                let expected_str = details.expected_interface.borrow().name.as_str().to_string();
                let actual_str = details.actual_type.to_string();

                match details.actual_type.is_undefined() {
                    true => None,
                    false => Some(format!("type `{}` does not match interface `{}`", actual_str, expected_str)),
                }
            },
            CompilationErrorDetails::UnexpectedExpression => {
                Some(format!("unexpected expression"))
            },
            CompilationErrorDetails::UnexpectedKeyword(details) => {
                Some(format!("unexpected keyword `{}`", details.keyword))
            },
            CompilationErrorDetails::UnexpectedVoidExpression => {
                Some(format!("expected non-void expression"))
            },
            CompilationErrorDetails::InvalidCharacter(details) => {
                Some(format!("invalid character '{}'", details.character))
            },
            CompilationErrorDetails::ExpectedExpression => {
                Some(format!("expected expression"))
            },
            CompilationErrorDetails::ExpectedClassType(details) => {
                Some(format!("expected class type, got `{}`", &details.actual_type))
            },
        }
    }

    // fn get_details(&self) -> Vec<String> {
    //     match &self.details {
    //         CompilationErrorDetails::ParseError(_) => vec![],
    //         CompilationErrorDetails::TypeMismatch(_) => vec![],
    //     }
    // }
}