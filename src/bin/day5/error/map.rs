/// MapType Error Codes
/// MapType -> MapError error translation
///
use thiserror::Error;
use super::MapError;

#[derive(Debug,Error)]
pub enum MapTypeError {
    #[error("Map Type [{0}] neither of seed, soil, fertilizer, water, light, temperature, humidity, location")]
    UnknownMapType(String)
}

impl From<MapTypeError> for MapError {
    fn from(err: MapTypeError) -> Self {
        MapError::MapTypeError(err.to_string())
    }
}
