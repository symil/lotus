use std::{ops::Deref, mem::take};
use parsable::{ItemLocation, ParseError, Parsable};
use crate::{utils::{Link, is_valid_identifier}, items::{Identifier, Word}};
use super::{CompilationError, CompilationErrorDetails, GenericErrorDetails, ParseErrorDetails, Type, TypeMismatchDetails, InterfaceBlueprint, InterfaceMismatchDetails, InvalidCharacterDetails, ExpectedClassTypeDetails, UndefinedItemDetails, ItemKind, UnexpectedTokenDetails, ExpectedKind, ExpectedTokenDetails, CompilationErrorChain};

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

    pub fn generic(&mut self, location: &ItemLocation, error: String) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::Generic(GenericErrorDetails {
                error,
            }),
        })
    }

    pub fn parse_error(&mut self, parse_error: &ParseError) -> CompilationErrorChain {
        self.add(CompilationError {
            location: ItemLocation {
                file: parse_error.file.clone(),
                start: parse_error.index,
                end: parse_error.index,
            },
            details: CompilationErrorDetails::ParseError(ParseErrorDetails {
                expected_tokens: parse_error.expected.clone()
            }),
        })
    }

    pub fn type_mismatch(&mut self, location: &ItemLocation, expected_type: &Type, actual_type: &Type) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::TypeMismatch(TypeMismatchDetails {
                expected_type: expected_type.clone(),
                actual_type: actual_type.clone(),
            }),
        })
    }

    pub fn interface_mismatch(&mut self, location: &ItemLocation, expected_interface: &Link<InterfaceBlueprint>, actual_type: &Type) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InterfaceMismatch(InterfaceMismatchDetails {
                expected_interface: expected_interface.clone(),
                actual_type: actual_type.clone(),
            }),
        })
    }

    pub fn unexpected_expression(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedToken(UnexpectedTokenDetails {
                kind: ExpectedKind::Expression,
                value: None,
            }),
        })
    }

    pub fn unexpected_void_expression(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedVoidExpression,
        })
    }

    pub fn unexpected_keyword(&mut self, location: &ItemLocation, keyword: &str) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::UnexpectedToken(UnexpectedTokenDetails {
                kind: ExpectedKind::Keyword,
                value: Some(keyword.to_string()),
            }),
        })
    }

    pub fn invalid_character(&mut self, location: &ItemLocation, character: &str) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::InvalidCharacter(InvalidCharacterDetails {
                character: character.to_string(),
            }),
        })
    }

    pub fn expected_type(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.add(CompilationError {
            location: location.clone(),
            details: CompilationErrorDetails::ExpectedToken(ExpectedTokenDetails {
                kind: ExpectedKind::Type,
            })
        })
    }

    pub fn expected_class_type(&mut self, location: &ItemLocation, actual_type: &Type) -> CompilationErrorChain {
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

    pub fn expected_identifier(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Identifier, is_valid_identifier(location.as_str()))
    }

    pub fn expected_expression(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Expression, is_valid_identifier(location.as_str()))
    }

    pub fn expected_function_body(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::FunctionBody, false)
    }

    pub fn expected_block(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Block, false)
    }

    pub fn expected_argument(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Argument, is_valid_identifier(location.as_str()))
    }

    pub fn expected_keyword(&mut self, location: &ItemLocation, keyword: &'static str) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Token(keyword), is_valid_identifier(location.as_str()))
    }

    pub fn expected_keyword_among(&mut self, location: &ItemLocation, keyword_list: &[&'static str]) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::TokenAmong(keyword_list.to_vec()), is_valid_identifier(location.as_str()))
    }

    pub fn expected_token(&mut self, location: &ItemLocation, token: &'static str) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Token(token), false)
    }

    pub fn expected_item<T : Parsable>(&mut self, location: &ItemLocation) -> CompilationErrorChain {
        self.expected(location, ExpectedKind::Item(T::get_item_name()), false)
    }

    pub fn keyword_mismatch(&mut self, word: &Word, expected: &[&'static str]) -> CompilationErrorChain {
        self.add(CompilationError {
            location: word.location.clone(),
            details: CompilationErrorDetails::ExpectedToken(ExpectedTokenDetails {
                kind: ExpectedKind::TokenAmong(expected.to_vec()),
            }),
        })
    }

    fn expected(&mut self, location: &ItemLocation, token: ExpectedKind, add_offset: bool) -> CompilationErrorChain {
        let final_location = match add_offset {
            true => location.get_end().set_start_with_offset(1),
            false => location.get_end(),
        };

        self.add(CompilationError {
            location: final_location,
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