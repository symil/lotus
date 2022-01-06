use std::collections::HashSet;
use colored::Colorize;
use parsable::{DataLocation, ParseError};
use crate::utils::Link;
use super::{InterfaceBlueprint, Type, ItemKind, TokenKind};

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
    ExpectedToken(ExpectedTokenDetails),
    UnexpectedToken(UnexpectedTokenDetails),
    UnexpectedVoidExpression,
    InvalidCharacter(InvalidCharacterDetails),
    ExpectedClassType(ExpectedClassTypeDetails),
    UndefinedItem(UndefinedItemDetails)
}

#[derive(Debug)]
pub struct ExpectedTokenDetails {
    pub kind: TokenKind,
}

#[derive(Debug)]
pub struct UnexpectedTokenDetails {
    pub kind: TokenKind,
    pub value: Option<String>
}

#[derive(Debug)]
pub struct UndefinedItemDetails {
    pub kind: ItemKind,
    pub name: String
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
pub struct InvalidCharacterDetails {
    pub character: String
}

impl CompilationError {
    pub fn to_string(&self) -> Option<String> {
        match self.get_message() {
            Some(first_line) => {
                let error_string = format!("{} {}", "error:".red().bold(), first_line);
                let (line, col) = self.location.get_line_col();
                let file_name = match self.location.file.path.starts_with(self.location.file.package_root_path.as_str()) {
                    true => &self.location.file.path[(self.location.file.package_root_path.len() + 1)..],
                    false => self.location.file.path.as_str(),
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
            CompilationErrorDetails::ExpectedToken(details) => {
                Some(format!("expected {}", details.kind.to_str()))
            },
            CompilationErrorDetails::UnexpectedToken(details) => {
                let mut result = format!("unexpected {}", details.kind.to_str());

                if let Some(name) = &details.value {
                    result.push_str(&format!(" `{}`", name));
                }

                Some(result)
            },
            CompilationErrorDetails::UnexpectedVoidExpression => {
                Some(format!("expected non-void expression"))
            },
            CompilationErrorDetails::InvalidCharacter(details) => {
                Some(format!("invalid character '{}'", details.character))
            },
            CompilationErrorDetails::ExpectedClassType(details) => {
                Some(format!("expected class type, got `{}`", &details.actual_type))
            },
            CompilationErrorDetails::UndefinedItem(details) => {
                Some(format!("undefined {} `{}`", details.kind.to_str(), details.name.bold()))
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