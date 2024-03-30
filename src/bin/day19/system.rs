use crate::part::Unit;
use crate::workflow::Workflow;
use crate::{
    part::Part,
    rule::{Action, Rule},
};
use std::collections::VecDeque;
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

        print!("{:?}: {} -> ", part, wf.name);
        while let Some(Action::WorkFlow(next)) = wf.validate(part) {
            wf = self
                .map
                .get(&next)
                .expect("SortingSystem::process() - redirected to non-existent Workflow");
            print!("{:?} -> ", wf.name);
        }
        let out = wf.validate(part);
        println!("{:?}", out.as_ref().unwrap());
        out
    }

    pub(crate) fn total_combinations(&self, wf: &str, rngs: &[Range<Unit>; 4], tab:usize) -> Unit {
        let mut queue: VecDeque<_> = VecDeque::new();

        print!("\n{:->tab$}:{:?} -> ",wf,rngs);
        self.map
            .get(wf.into())
            .unwrap()
            .iter()
            .collect_into(&mut queue);

        let mut prng = rngs.clone();
        let mut sum = 0;
        while let Some(rule) = queue.pop_front() {
            print!("\n{0:->tab$} ",rule);
            sum += match rule {
                Rule::ConAct(c, a) => {
                    let part = c.part() as usize;
                    let Some((result,remainder)) = c.partition(&prng[part]) else { panic!("Ops") };
                    prng[part] = remainder;
                    let mut tmp = prng.clone();
                    tmp[part] = result;
                    match a {
                        Action::WorkFlow(next_wf) => self
                            .total_combinations(next_wf, &tmp, tab+4),
                        Action::Accept => tmp
                            .iter()
                            .map(|r| r.len() as Unit)
                            .inspect(|d| print!("{d},"))
                            .product(),
                        Action::Reject => 0,
                    }
                },
                Rule::Act(a) => {
                    match a {
                        Action::WorkFlow(next_wf) => self
                            .total_combinations(next_wf, &prng, tab+4),
                        Action::Accept => prng
                            .iter()
                            .map(|r| r.len() as Unit)
                            .inspect(|d| print!("{d},"))
                            .product(),
                        Action::Reject => 0,
                    }
                }
            };
        }
        println!(" = {sum} ({wf})");
        sum
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
        let (_, wfs) = parse_puzzle_data("src/bin/day19/sample1.txt");
        wfs.total_combinations("in", &[1..4001, 1..4001, 1..4001, 1..4001], 0);
        // 132_753_196_000_000
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
