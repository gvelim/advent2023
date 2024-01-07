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
            // .inspect(|p| print!("{:?} -> ",&p))
            .map(|(v,h)| {
                v.unwrap_or((0,0)).0 * 100 + h.unwrap_or((0,0)).0
            })
            // .inspect(|p| println!("{:?}",&p))
            .sum::<usize>()
    }
    pub(crate) fn summarise_smudged(&mut self) -> usize {
        self.patterns.iter()
            .map(|pat| {
                let v = Pattern::find_smudge(&pat.t).max();
                let h = Pattern::find_smudge(&pat.p).max();
                (v,h)
            })
            // .inspect(|p| print!("{:?} -> ",&p))
            .map(|(v,h)|
                    v.unwrap_or((0,0,0)).0 * 100 + h.unwrap_or((0,0,0)).0
            )
            // .inspect(|p| println!("{:?}",&p))
            .sum::<usize>()
    }}
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
