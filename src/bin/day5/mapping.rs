use std::{num::ParseIntError, ops::Range, str::FromStr};

#[derive(Debug,PartialEq)]
pub(crate) struct Mapping {
    pub src_base: Range<u64>, // 98 (98,99)
    pub dst_base: u64, // 52
}

impl Mapping {
    #[inline]
    fn shift(&self, n:u64) ->u64 {
        self.dst_base + n - self.src_base.start
    }

    pub(crate) fn transform(&self, seed: u64) -> Option<u64> {
        if self.src_base.contains(&seed) {
            Some(self.shift(seed))
        } else {
            None
        }
    }

    pub(crate) fn transform_range(&self, rng: &Range<u64>) -> (Option<Range<u64>>,Vec<Range<u64>>) {
        let src = &self.src_base;
        match (src.contains(&rng.start), src.contains(&(rng.end-1))) {
            (true, true) =>
                // src range contains input range
                (Some(self.shift(rng.start)..self.shift(rng.end)), vec![]),
            (true, false) =>
                // overlapping right of src
                (Some(self.shift(rng.start)..self.shift(src.end)), vec![src.end..rng.end]),
            (false, true) =>
                // overlapping left of src
                (Some(self.shift(src.start)..self.shift(rng.end)), vec![rng.start..src.start]),
            (false, false) =>{
                // does it fall left or right of the src range ?
                if rng.end <= src.start || rng.start >= src.end {
                    (None, vec![rng.clone()])
                } else {
                    // input range contains the src range, hence
                    (Some(self.shift(src.start)..self.shift(src.end)), vec![rng.start..src.start,src.end..rng.end])
                }
            }
        }
    }
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

#[cfg(test)]
mod test {
    use super::Mapping;

    #[test]
    fn test_mapping_transform_range() {
        let data = [
            ("50 98 2", 79..98, (Some(79..98), vec![])),
            ("52 50 48", 79..98, (Some(81..100), vec![])),
            ("0 15 37", 42..62, (Some(27..37), vec![52..62])),
            ("37 52 2", 42..53, (Some(37..38), vec![42..52])),
            ("39 5 15", 0..30, (Some(39..54), vec![0..5, 20..30]))
        ];

        for (inp,rng,out) in data {
            let mapping = inp.parse::<Mapping>().expect("Ops");
            let ret = mapping.transform_range(&rng);
            println!("{:?} : {:?} = {:?}",mapping, rng, ret);
            assert_eq!(ret,out)
        }
    }

    #[test]
    fn test_mapping_transform() {
        let data = [
            ("50 98 2",100,None),
            ("52 50 48",79,Some(81)),
            ("0 15 37",15,Some(0)),
            ("37 52 2",54,None),
            ("39 0 15",14,Some(53))
        ];

        for (inp,seed, out) in data {
            let mapping = inp.parse::<Mapping>().expect("Ops");
            println!("{:?} : {seed} = {:?}",mapping,mapping.transform(seed));
            assert_eq!(
                mapping.transform(seed),
                out
            )
        }
    }
}
