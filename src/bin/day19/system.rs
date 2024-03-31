use crate::part::Unit;
use crate::workflow::Workflow;
use crate::{
    part::Part,
    rule::{Action, Rule},
};
use std::ops::Range;
use std::{collections::HashMap, rc::Rc, str::FromStr};

pub(crate) struct SortingSystem {
    map: HashMap<Rc<str>, Workflow>
}

impl SortingSystem {
    pub(crate) fn process_part(&self, part: Part, workflow: &str) -> Option<Action> {
        // If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns.
        // If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.
        let mut wf = self
            .map
            .get(workflow.into())
            .expect("SortingSystem::process() - Starting workflow unknown!!");

        // print!("{:?}: {} -> ", part, wf.name);
        while let Some(Action::WorkFlow(next)) = wf.validate(part) {
            wf = self
                .map
                .get(&next)
                .expect("SortingSystem::process() - redirected to non-existent Workflow");
            // print!("{:?} -> ", wf.name);
        }
        let out = wf.validate(part);
        // println!("{:?}", out.as_ref().unwrap());
        out
    }

    pub(crate) fn total_combinations(&self, wf: &str, rngs: &[Range<Unit>; 4], tab:usize) -> Unit {
        let mut remain = rngs.clone();
        // print!("\n{:->tab$}:{:?} -> ",wf,rngs);

        self.map
            .get(wf.into())
            .unwrap()
            .iter()
            .map(|rule| {
                // print!("\n{:tab$}{rule}","");
                match rule {
                    Rule::ConAct(c, a) => {
                        let part = c.part() as usize;
                        let mut result = remain.clone();
                        (result[part], remain[part]) = c.partition(&remain[part]);
                        match a {
                            Action::WorkFlow(next_wf) => self
                                .total_combinations(next_wf, &result, tab+4),
                            Action::Accept => result
                                .iter()
                                .map(|r| r.len() as Unit)
                                // .inspect(|d| print!("{d},"))
                                .product(),
                            Action::Reject => 0,
                        }
                    },
                    Rule::Act(a) => {
                        match a {
                            Action::WorkFlow(next_wf) => self
                                .total_combinations(next_wf, &remain, tab+4),
                            Action::Accept => remain
                                .iter()
                                .map(|r| r.len() as Unit)
                                // .inspect(|d| print!("{d},"))
                                .product(),
                            Action::Reject => 0,
                        }
                    }
                }
            })
            // .inspect(|sum| println!("{:tab$}= {sum} ({wf})",""))
            .sum::<Unit>()
    }
}

impl FromStr for SortingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Rc<str>, Workflow> = HashMap::new();

        for line in s.lines() {
            let wf = line.parse::<Workflow>()?;
            map.insert(wf.key(), wf);
        }
        Ok(SortingSystem { map })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse_puzzle_data;

    #[test]
    fn test_sortingsystem_combinations() {
        let (_, wfs) = parse_puzzle_data("src/bin/day19/sample.txt");
        let sum = wfs.total_combinations("in", &[1..4001, 1..4001, 1..4001, 1..4001], 0);
        println!("Total combinations: {sum}");
        // assert_eq!(sum,132_753_196_000_000);
        assert_eq!(sum,167_409_079_868_000);
    }

    #[test]
    fn test_sortingsystem_process() {
        let (part, wfs) = parse_puzzle_data("src/bin/day19/sample.txt");

        let sum = part
            .iter()
            .filter(|&&part| wfs.process_part(part, "in") == Some(Action::Accept))
            .map(|p| p.sum())
            .sum::<Unit>();

        assert_eq!(sum, 19114);
    }

    #[test]
    fn test_sortingsystem_parse() {
        let inp =
            std::fs::read_to_string("src/bin/day19/sample.txt").expect("cannot load sample.txt");
        let wfs = inp.split("\n\n").next().unwrap();
        let sorting = wfs
            .parse::<SortingSystem>()
            .expect("Failed to parse workflow");

        wfs.lines().for_each(|line| {
            let wf = line.parse::<Workflow>().expect("msg");
            let found = sorting.map.get(&wf.key());
            println!("{:?}", found);
            assert_eq!(format!("{:?}", found), format!("{:?}", Some(wf)));
        });
    }
}
