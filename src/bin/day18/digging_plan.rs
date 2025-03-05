use crate::instruction::{Direction, Instruction, InstructionErr};
use std::str::FromStr;

pub(crate) struct DigPlan {
    pub(crate) set: std::rc::Rc<[Instruction]>,
}

impl DigPlan {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Instruction> + '_ {
        self.set.iter()
    }

    pub fn _is_clockwise(&self) -> bool {
        let mut last: Option<Direction> = None;
        self.set
            .iter()
            .map(|i| {
                let out = last
                    .map(|ld| if i.dir.is_clockwise(ld) { 1 } else { -1 })
                    .unwrap_or(0);
                last = Some(i.dir);
                out
            })
            .sum::<isize>()
            .is_positive()
    }
}

impl FromStr for DigPlan {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = vec![];
        for line in s.lines() {
            set.push(line.parse::<Instruction>()?);
        }
        Ok(DigPlan { set: set.into() })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lagoon::test::load_plan;

    #[test]
    fn test_digplan_parse() {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("Ops!");
        let plan = match inp.parse::<DigPlan>() {
            Ok(set) => set,
            Err(e) => panic!("{}", e),
        };

        let mut iter = plan.iter();
        inp.lines()
            .for_each(|line| {
                let out = &format!("{:?}",iter.next().unwrap());
                println!("{line} => {out}");
                assert_eq!(line,out);
            });
    }

    #[test]
    fn test_digplan_is_clockwise() {
        let plan: DigPlan = match load_plan(None) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        match plan._is_clockwise() {
            true => println!("Clockwise"),
            false => println!("Counter Clockwise"),
        }
    }
}
