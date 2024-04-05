use std::{num::ParseIntError, ops::Range, str::FromStr};

#[derive(Debug,PartialEq)]
pub(crate) struct Mapping {
    pub(crate) src_base: Range<u64>, // 98 (98,99)
    pub(crate) dst_base: u64, // 52
}

impl FromStr for Mapping {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let dst_base = nums.next().expect("Mapping: Missing value").parse::<u64>()?;
        let src_base = nums.next().expect("Mapping: Missing value").parse::<u64>()?;
        let len = nums.next().expect("Mapping: Missing value").parse::<u64>()?;

        Ok( Mapping {
            dst_base,
            src_base: (src_base..src_base + len),
        })
    }
}
