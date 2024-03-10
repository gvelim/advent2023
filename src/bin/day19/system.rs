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
        let mut wf = self.map
            .get(workflow.into())
            .expect("SortingSystem::process() - Starting workflow unknown!!");

        while let Some(Action::WorkFlow(next)) = wf.validate(part) {
            wf = self.map
                .get(&next)
                .expect("SortingSystem::process() - redirected to non-existent Workflow");
        }

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

    #[test]
    fn test_sortingsystem_process() {
        let inp = std::fs::read_to_string("src/bin/day19/sample.txt")
            .expect("cannot load sample.txt");
        let mut split = inp.split("\n\n");
        let wfs = split
            .next()
            .unwrap()
            .parse::<SortingSystem>()
            .expect("Failed to parse workflow");
        let part = "{x=787,m=2655,a=1222,s=2876}".parse::<Part>().expect("msg");

        println!("{:?}", wfs.process_part(part, "in"));
        assert_eq!(
            format!("{:?}", wfs.process_part(part, "in")),
            format!("{:?}", Some(Action::Accept))
        );
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
