/// Mapping Error Codes
/// Mapping -> MapError error translation

use std::num::ParseIntError;
use thiserror::Error;
use super::MapError;

#[derive(Debug,PartialEq,Error)]
pub enum MappingError {
    #[error("Missing Mapping value: [{0}]")]
    MappingValueMissing(String),
    #[error("Invalid Mapping value: [{0}]")]
    MappingValueInvalid(String)
}

impl From<ParseIntError> for MappingError {
    fn from(err: ParseIntError) -> Self {
        MappingError::MappingValueInvalid(err.to_string())
    }
}

impl From<MappingError> for MapError {
    fn from(err: MappingError) -> Self {
        MapError::MappingError(err.to_string())
    }
}
