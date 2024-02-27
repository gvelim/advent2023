use std::str::FromStr;
use crate::instruction::{Instruction, InstructionErr};

pub(crate) struct DigPlan {
    pub(crate) set: std::rc::Rc<[Instruction]>
}

impl DigPlan {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Instruction> + '_ {
        self.set.iter()
    }
}

impl FromStr for DigPlan {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = vec![];
        for line in s.lines() {
            set.push( line.parse::<Instruction>()? );
        }
        Ok(DigPlan {
            set: set.into()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digplan_parse()  {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("Ops!");
        let plan = match inp.parse::<DigPlan>() {
            Ok(set) => set,
            Err(e) => panic!("{}",e),
        };

        let mut iter = plan.iter();
        inp.lines()
            .for_each(|line| {
                let out = &format!("{:?}",iter.next().unwrap());
                println!("{line} => {out}");
                assert_eq!(line,out);
            });
    }
}
