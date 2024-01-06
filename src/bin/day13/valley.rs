use std::str::FromStr;
use crate::pattern::Pattern;

#[derive(Debug)]
pub(crate) struct Valley {
    pub(crate) patterns: Vec<Pattern>
}
impl Valley {
    pub(crate) fn summarise_notes(&mut self) -> usize {
        self.patterns.iter_mut()
            .map(|pat| {
                let v = pat.find_vertical_mirror_max();
                let h = pat.find_horizontal_mirror_max();
                (v,h)
            })
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|(v,h)| {
                match (v,h) {
                    (Some(v), Some(h)) => if v.1 > h.1 { v.0 * 100 } else { h.0 },
                    (Some(v), None) => v.0 * 100,
                    (None, Some(h)) => h.0,
                    (None,None) => 0
                }
            })
            .inspect(|p| println!("{:?}",&p))
            .sum::<usize>()
    }
    pub(crate) fn fix_smudged_mirrors(&mut self) {
        self.patterns.iter_mut()
            .for_each(|pat|{
                pat.fix_smudge().expect("Ops! No smudge found for pattern");
            })
    }
}
impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Valley {
            patterns: s.split("\n\n")
                .map(|pat| pat.parse::<Pattern>().expect("Ops!"))
                .collect::<Vec<_>>()
        })
    }
}
