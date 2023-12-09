use std::{ops::Range, str::FromStr};

#[derive(Debug,PartialEq)]
pub(crate) struct Mapping {
    pub(crate) src_base: Range<u64>, // 98 (98,99)
    pub(crate) dst_base: u64, // 52
    pub(crate) len: u64
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let dst_base = u64::from_str(nums.next().unwrap()).expect("dst_base::Ops!");
        let src_base = u64::from_str(nums.next().unwrap()).expect("src_base::Ops!");
        let len = u64::from_str(nums.next().unwrap()).expect("len::Ops!");

        Ok( Mapping {
            dst_base,
            src_base: (src_base..src_base + len),
            len
        })
    }
}