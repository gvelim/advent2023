use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub(crate) struct Numbers(pub(crate) HashSet<u32>);
impl FromStr for Numbers {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok( Numbers( input
            .split_ascii_whitespace()
            .map(|num| u32::from_str(num).expect("Ops!"))
            .collect::<HashSet<u32>>()
        ))
    }
}