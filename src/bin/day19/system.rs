use std::{collections::HashMap, rc::Rc, str::FromStr};
use crate::workflow::Workflow;
use crate::{part::Part, rule::Action};

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

        println!("{:?}", &wf.validate(part));
        wf.validate(part)
    }
}

impl FromStr for SortingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Rc<str>,Workflow> = HashMap::new();

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
    use crate::{parse_puzzle_data, part::Unit};

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
        let inp = std::fs::read_to_string("src/bin/day19/sample.txt")
            .expect("cannot load sample.txt");
        let wfs = inp.split("\n\n").next().unwrap();
        let sorting = wfs.parse::<SortingSystem>()
            .expect("Failed to parse workflow");

        wfs.lines()
            .for_each(|line|{
                let wf = line.parse::<Workflow>().expect("msg");
                let found = sorting.map.get(&wf.key());
                println!("{:?}",found);
                assert_eq!(
                    format!("{:?}", found),
                    format!("{:?}", Some(wf))
                );
            });
    }
}
