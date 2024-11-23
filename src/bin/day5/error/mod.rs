use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug,PartialEq,Error)]
pub enum MappingError {
    #[error("Mapping value is missing or too few provided")]
    MappingValueMissing,
    #[error("Non numeric Mapping value found or value out of 32bit range")]
    MappingValueInvalid
}

impl From<ParseIntError> for MappingError {
    fn from(_: ParseIntError) -> Self {
        MappingError::MappingValueInvalid
    }
}

#[derive(Debug,Error)]
pub enum MapTypeError {
    #[error("Map Type provided not in [seed, soil, fertilizer, water, light, temperature, humidity, location]")]
    UnknownMapType
}

#[derive(Debug,PartialEq,Error)]
pub enum MapError {
    #[error("Map Type provided not in [seed, soil, fertilizer, water, light, temperature, humidity, location]")]
    InvalidMapType,
    #[error("Map Type is missing; either one or no map types provided")]
    MissingMapType,
    #[error("Map Type contains a non-numeric value")]
    InvalidMappingValues,
    #[error("Map input doesn't appear to fit the desire format; cannot find <map type>-to-<map type>")]
    ParseInputFormatInvalid
}

impl From<MappingError> for MapError {
    fn from(err: MappingError) -> Self {
        match err {
            MappingError::MappingValueMissing => MapError::InvalidMappingValues,
            MappingError::MappingValueInvalid => MapError::InvalidMappingValues,
        }
    }
}
impl From<MapTypeError> for MapError {
    fn from(_: MapTypeError) -> Self {
        MapError::InvalidMapType
    }
}
