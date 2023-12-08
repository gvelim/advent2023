use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub(crate) struct Numbers(pub(crate) HashSet<u32>);
impl FromStr for Numbers {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok( Numbers( input
            .split(' ')
            .filter(|&d| !d.is_empty())
            .map(|num| u32::from_str(num.trim()).expect("Ops!"))
            .collect::<HashSet<u32>>()
        ))
    }
}
