use std::str::FromStr;
use crate::pattern::Pattern;

#[derive(Debug)]
pub(crate) struct Valley {
    pub(crate) patterns: Vec<Pattern>
}
impl Valley {
    pub(crate) fn summarise_notes(&self) -> usize {
        self.patterns.iter()
            .map(|pat| {
                (pat.find_vertical_mirror(), pat.find_horizontal_mirror())
            })
            .map(|(v,h)| {
                v.unwrap_or((0,0)).0 * 100 + h.unwrap_or((0,0)).0
            })
            .sum::<usize>()
    }
    pub(crate) fn summarise_smudged(&self) -> usize {
        self.patterns.iter()
            .map(|pat| {
                (Pattern::find_smudged_reflection(&pat.t).next(), Pattern::find_smudged_reflection(&pat.p).next())
            })
            .map(|(v,h)|
                    v.unwrap_or((0,0,0)).0 * 100 + h.unwrap_or((0,0,0)).0
            )
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_smudged_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_smudged(), 400);
    }

    #[test]
    fn test_calculate_sample_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_notes(), 405);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }

}