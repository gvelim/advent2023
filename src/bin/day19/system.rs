use crate::part::Unit;
use crate::workflow::Workflow;
use crate::{
    part::Part,
    rule::{Action, Rule},
};
use std::ops::Range;
use std::{collections::HashMap, rc::Rc, str::FromStr};

pub(crate) struct SortingSystem {
    map: HashMap<Rc<str>, Workflow>,
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

    pub(crate) fn rating_combinations(&self, wf: &str, rngs: &[Range<Unit>; 4]) -> Option<[Range<Unit>; 4]> {
        self.map
            .get(wf.into())
            .unwrap()
            .iter()
            // .inspect(|arr| println!("{:?} -> ", arr))
            .filter_map(|rule| match rule {
                Rule::ConAct(c, a) => {
                    let mut nrng = rngs.clone();
                    if let Some((drng, _)) = rule.validate_range(&rngs[c.part() as usize]) {
                        nrng[c.part() as usize] = drng;
                    }
                    match a {
                        Action::WorkFlow(wf) => self.rating_combinations(wf, &nrng),
                        Action::Accept => Some(nrng),
                        Action::Reject => Some(nrng),
                    }
                }
                Rule::Act(Action::WorkFlow(wf)) => self.rating_combinations(wf, rngs),
                Rule::Act(Action::Accept) => Some(rngs.clone()),
                Rule::Act(Action::Reject) => None,
            })
            .inspect(|arr| println!(" = {:?}", arr))
            .last()
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
        wfs.rating_combinations("in", &[0..4000, 0..4000, 0..4000, 0..4000]);
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
