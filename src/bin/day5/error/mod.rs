mod map;
mod mapping;

pub use map::*;
pub use mapping::*;

use thiserror::Error;

// Top level Error exposed to main
#[derive(Debug,PartialEq,Error)]
pub enum MapError {
    #[error("Map Type Error: [{0}]")]
    MapTypeError(String),
    #[error("Mapping Error: [{0}]")]
    MappingError(String),
    #[error("Map input doesn't appear to fit the desire format; cannot find <map type>-to-<map type>")]
    ParseInputFormatInvalid
}
