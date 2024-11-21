use std::{
    collections::HashSet,
    num::ParseIntError,
    str::FromStr
};

#[derive(Debug)]
pub(crate) struct Numbers(pub(crate) HashSet<u32>);
impl FromStr for Numbers {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input
            .split_ascii_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<u32>,_>>()
        {
            Ok(set) =>  Ok(Numbers(set)),
            Err(e) => Err(e),
        }
    }
}
