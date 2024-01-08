use std::rc::Rc;
use std::str::FromStr;
use crate::pattern::{Reflection, Pattern};

#[derive(Debug)]
pub(crate) struct Valley {
    pub(crate) patterns: Rc<[Pattern]>
}
impl Valley {
    pub(crate) fn summarise_notes<'a, F, I>(&'a self, find: F) -> usize
        where
            F: Fn(&'a [String]) -> I,
            I: Iterator<Item = Reflection> + 'a
    {
        self.patterns.iter()
            .map(|pat| (find(&pat.t).next(), find(&pat.p).next()) )
            .map(|(v,h)| {
                v.unwrap_or((0,0)).0 * 100 + h.unwrap_or((0,0)).0
            })
            .sum::<usize>()
    }
}
impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Valley {
            patterns: s.split("\n\n")
                .map(|pat| pat.parse::<Pattern>().expect("Ops!"))
                .collect::<Rc<[Pattern]>>()
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_smudged_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_notes(Pattern::find_smudged_reflection), 400);
    }

    #[test]
    fn test_calculate_sample_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_notes(Pattern::find_perfect_reflection), 405);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }

}