use std::collections::HashSet;
use colored::Colorize;
use parsable::{DataLocation, ParseError};
use crate::utils::Link;
use super::{InterfaceBlueprint, Type};

#[derive(Debug)]
pub struct CompilationError {
    location: Option<DataLocation>,
    details: CompilationErrorDetails,
}

#[derive(Debug)]
enum CompilationErrorDetails {
    Generic(GenericErrorDetails),
    ParseError(ParseErrorDetails),
    TypeMismatch(TypeMismatchDetails),
    InterfaceMismatch(InterfaceMismatchDetails),
    UnexpectedKeyword(UnexpectedKeywordDetails),
    ExpectedExpression,
    UnexpectedExpression,
    UnexpectedVoidExpression,
    InvalidCharacter(InvalidCharacterDetails)
}

#[derive(Debug)]
struct GenericErrorDetails {
    error: String
}

#[derive(Debug)]
struct ParseErrorDetails {
    expected_tokens: Vec<String>
}

#[derive(Debug)]
struct TypeMismatchDetails {
    expected_type: Type,
    actual_type: Type
}

#[derive(Debug)]
struct InterfaceMismatchDetails {
    expected_interface: Link<InterfaceBlueprint>,
    actual_type: Type
}

#[derive(Debug)]
struct UnexpectedKeywordDetails {
    keyword: String
}

#[derive(Debug)]
struct InvalidCharacterDetails {
    character: String
}

impl CompilationError {
    pub fn generic(location: &DataLocation, error: String) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::Generic(GenericErrorDetails {
                error,
            }),
        }
    }

    pub fn generic_unlocated(error: String) -> Self {
        Self {
            location: None,
            details: CompilationErrorDetails::Generic(GenericErrorDetails {
                error,
            }),
        }
    }

    pub fn parse_error(parse_error: ParseError) -> Self {
        Self {
            location: Some(DataLocation {
                package_root_path: parse_error.package_root_path,
                file_path: parse_error.file_path,
                file_content: parse_error.file_content,
                start: parse_error.index,
                end: parse_error.index,
            }),
            details: CompilationErrorDetails::ParseError(ParseErrorDetails {
                expected_tokens: parse_error.expected
            }),
        }
    }

    pub fn type_mismatch(location: &DataLocation, expected_type: &Type, actual_type: &Type) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::TypeMismatch(TypeMismatchDetails {
                expected_type: expected_type.clone(),
                actual_type: actual_type.clone(),
            }),
        }
    }

    pub fn interface_mismatch(location: &DataLocation, expected_interface: &Link<InterfaceBlueprint>, actual_type: &Type) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::InterfaceMismatch(InterfaceMismatchDetails {
                expected_interface: expected_interface.clone(),
                actual_type: actual_type.clone(),
            }),
        }
    }

    pub fn expected_expression(location: &DataLocation) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::ExpectedExpression,
        }
    }

    pub fn unexpected_expression(location: &DataLocation) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::UnexpectedExpression,
        }
    }

    pub fn unexpected_void_expression(location: &DataLocation) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::UnexpectedVoidExpression,
        }
    }

    pub fn unexpected_keyword(location: &DataLocation, keyword: &str) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::UnexpectedKeyword(UnexpectedKeywordDetails {
                keyword: keyword.to_string(),
            }),
        }
    }

    pub fn invalid_character(location: &DataLocation, character: &str) -> Self {
        Self {
            location: Some(location.clone()),
            details: CompilationErrorDetails::InvalidCharacter(InvalidCharacterDetails {
                character: character.to_string(),
            }),
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self.get_first_line() {
            Some(first_line) => {
                let error_string = format!("{} {}", "error:".red().bold(), first_line);
                let location_string = match &self.location {
                    Some(location) => {
                        let (line, col) = location.get_line_col();

                        format!("{}:{}:{}: ", location.file_path, line, col)
                    },
                    None => String::new(),
                };

                let mut result = format!("{}{}", location_string.bold(), error_string);

                // for detail in self.get_details() {
                //     result.push_str(&format!("\n  - {}", detail));
                // }

                Some(result)
            },
            None => None,
        }
    }

    fn get_first_line(&self) -> Option<String> {
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
                let expected_str = details.expected_type.to_string().bold();
                let actual_str = details.actual_type.to_string().bold();

                match details.actual_type.is_undefined() {
                    true => None,
                    false => Some(format!("expected `{}`, got `{}`", expected_str, actual_str))
                }
            }
            CompilationErrorDetails::InterfaceMismatch(details) => {
                let expected_str = details.expected_interface.borrow().name.as_str().bold();
                let actual_str = details.actual_type.to_string().bold();

                match details.actual_type.is_undefined() {
                    true => None,
                    false => Some(format!("type `{}` does not match interface `{}`", actual_str, expected_str)),
                }
            },
            CompilationErrorDetails::UnexpectedExpression => {
                Some(format!("unexpected expression"))
            },
            CompilationErrorDetails::UnexpectedKeyword(details) => {
                Some(format!("unexpected keyword `{}`", details.keyword.bold()))
            },
            CompilationErrorDetails::UnexpectedVoidExpression => {
                Some(format!("expected non-void expression"))
            },
            CompilationErrorDetails::InvalidCharacter(details) => {
                Some(format!("invalid character '{}'", details.character.bold()))
            },
            CompilationErrorDetails::ExpectedExpression => {
                Some(format!("expected expression"))
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