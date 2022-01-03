use std::{ops::Deref, mem::take};
use parsable::{DataLocation, ParseError};
use crate::{utils::Link, items::Identifier};

use super::{CompilationError, CompilationErrorDetails, GenericErrorDetails, ParseErrorDetails, Type, TypeMismatchDetails, InterfaceBlueprint, InterfaceMismatchDetails, InvalidCharacterDetails, ExpectedClassTypeDetails, UndefinedItemDetails, ItemKind, UnexpectedTokenDetails, TokenKind, ExpectedTokenDetails, CompilationErrorChain};

#[derive(Debug)]
pub struct CompilationErrorList {
    errors: Vec<CompilationError>,
    enabled: bool
}

impl CompilationErrorList {
    pub fn new() -> Self {
        Self {
            errors: vec![],
            enabled: true,
        }
    }

    pub fn add(&mut self, error: CompilationError) -> CompilationErrorChain {
        if self.enabled {
            self.errors.push(error)
        }

        CompilationErrorChain
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn get_all(&self) -> &[CompilationError] {
        &self.errors
    }

    pub fn generic(&mut self, location: &DataLocation, error: String) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::Generic(GenericErrorDetails {
                error,
            }),
        })
    }

    pub fn parse_error(&mut self, parse_error: &ParseError) -> CompilationErrorChain {
        self.add(CompilationError {
            location: DataLocation {
                package_root_path: parse_error.package_root_path.clone(),
                file_path: parse_error.file_path.clone(),
                file_content: parse_error.file_content.clone(),
                start: parse_error.index,
                end: parse_error.index,
            },
            details: CompilationErrorDetails::ParseError(ParseErrorDetails {
                expected_tokens: parse_error.expected.clone()
            }),
        })
    }

    pub fn type_mismatch(&mut self, location: &DataLocation, expected_type: &Type, actual_type: &Type) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::TypeMismatch(TypeMismatchDetails {
                expected_type: expected_type.clone(),
                actual_type: actual_type.clone(),
            }),
        })
    }

    pub fn interface_mismatch(&mut self, location: &DataLocation, expected_interface: &Link<InterfaceBlueprint>, actual_type: &Type) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InterfaceMismatch(InterfaceMismatchDetails {
                expected_interface: expected_interface.clone(),
                actual_type: actual_type.clone(),
            }),
        })
    }

    pub fn unexpected_expression(&mut self, location: &DataLocation) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedToken(UnexpectedTokenDetails {
                kind: TokenKind::Expression,
                value: None,
            }),
        })
    }

    pub fn unexpected_void_expression(&mut self, location: &DataLocation) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedVoidExpression,
        })
    }

    pub fn unexpected_keyword(&mut self, location: &DataLocation, keyword: &str) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedToken(UnexpectedTokenDetails {
                kind: TokenKind::Keyword,
                value: Some(keyword.to_string()),
            }),
        })
    }

    pub fn invalid_character(&mut self, location: &DataLocation, character: &str) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InvalidCharacter(InvalidCharacterDetails {
                character: character.to_string(),
            }),
        })
    }

    pub fn expected_class_type(&mut self, location: &DataLocation, actual_type: &Type) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::ExpectedClassType(ExpectedClassTypeDetails {
                actual_type: actual_type.clone(),
            })
        })
    }

    pub fn undefined_type(&mut self, identifier: &Identifier) -> CompilationErrorChain {
        self.add(CompilationError {
            location: identifier.location.clone(),
            details: CompilationErrorDetails::UndefinedItem(UndefinedItemDetails {
                kind: ItemKind::Type,
                name: identifier.to_string(),
            })
        })
    }

    pub fn expected_identifier(&mut self, location: &DataLocation) -> CompilationErrorChain {
        self.expected_token(location, TokenKind::Identifier)
    }

    pub fn expected_expression(&mut self, location: &DataLocation) -> CompilationErrorChain {
        self.expected_token(location, TokenKind::Expression)
    }

    pub fn expected_function_body(&mut self, location: &DataLocation) -> CompilationErrorChain {
        self.expected_token(location, TokenKind::FunctionBody)
    }

    fn expected_token(&mut self, location: &DataLocation, token: TokenKind) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.get_end(),
            details: CompilationErrorDetails::ExpectedToken(ExpectedTokenDetails {
                kind: token,
            }),
        })
    }
}

impl Default for CompilationErrorList {
    fn default() -> Self {
        Self::new()
    }
}