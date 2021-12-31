use std::{ops::Deref, mem::take};
use parsable::{DataLocation, ParseError};
use crate::utils::Link;

use super::{CompilationError, CompilationErrorDetails, GenericErrorDetails, ParseErrorDetails, Type, TypeMismatchDetails, InterfaceBlueprint, InterfaceMismatchDetails, UnexpectedKeywordDetails, InvalidCharacterDetails, ExpectedClassTypeDetails};

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

    pub fn add(&mut self, error: CompilationError) {
        if self.enabled {
            self.errors.push(error)
        }
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

    pub fn generic(&mut self, location: &DataLocation, error: String) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::Generic(GenericErrorDetails {
                error,
            }),
        })
    }

    pub fn parse_error(&mut self, parse_error: ParseError) {
        self.add(CompilationError {
            location: DataLocation {
                package_root_path: parse_error.package_root_path,
                file_path: parse_error.file_path,
                file_content: parse_error.file_content,
                start: parse_error.index,
                end: parse_error.index,
            },
            details: CompilationErrorDetails::ParseError(ParseErrorDetails {
                expected_tokens: parse_error.expected
            }),
        })
    }

    pub fn type_mismatch(&mut self, location: &DataLocation, expected_type: &Type, actual_type: &Type) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::TypeMismatch(TypeMismatchDetails {
                expected_type: expected_type.clone(),
                actual_type: actual_type.clone(),
            }),
        });
    }

    pub fn interface_mismatch(&mut self, location: &DataLocation, expected_interface: &Link<InterfaceBlueprint>, actual_type: &Type) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InterfaceMismatch(InterfaceMismatchDetails {
                expected_interface: expected_interface.clone(),
                actual_type: actual_type.clone(),
            }),
        });
    }

    pub fn expected_expression(&mut self, location: &DataLocation) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::ExpectedExpression,
        });
    }

    pub fn unexpected_expression(&mut self, location: &DataLocation) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedExpression,
        });
    }

    pub fn unexpected_void_expression(&mut self, location: &DataLocation) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedVoidExpression,
        });
    }

    pub fn unexpected_keyword(&mut self, location: &DataLocation, keyword: &str) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedKeyword(UnexpectedKeywordDetails {
                keyword: keyword.to_string(),
            }),
        });
    }

    pub fn invalid_character(&mut self, location: &DataLocation, character: &str) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InvalidCharacter(InvalidCharacterDetails {
                character: character.to_string(),
            }),
        });
    }

    pub fn expected_class_type(&mut self, location: &DataLocation, actual_type: &Type) {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::ExpectedClassType(ExpectedClassTypeDetails {
                actual_type: actual_type.clone(),
            })
        })
    }
}

impl Default for CompilationErrorList {
    fn default() -> Self {
        Self::new()
    }
}