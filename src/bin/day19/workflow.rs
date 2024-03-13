use std::{fmt::Debug, rc::Rc, str::FromStr};
use crate::{part::Part, rule::{Action, Rule}};

pub(crate) struct Workflow {
    // Each workflow has a name and contains a list of rules
    rules: Rc<[Rule]>,
    pub name: Rc<str>,
}

impl Workflow {
    pub(crate) fn key(&self) -> Rc<str> {
        self.name.clone()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
    pub(crate) fn validate(&self, part: Part) -> Option<Action> {
        // The first rule that matches the part being considered is applied immediately,
        // and the part moves on to the destination described by the rule
        self.iter()
            .skip_while(|rule| rule.validate(part).is_none())
            .map(|rule| rule.validate(part).unwrap() )
            .next()
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        // ex{ x>10:one ,m<20:two , a>30:R , A }
        let mut s = inp.split(&['{','}']);
        let name = s.next().unwrap().into();
        let rules = s.next().unwrap()
            .split(',')
            .map(|r| r.parse::<Rule>().expect("Workflow::from_str() invalid rule"))
            .collect::<Rc<_>>();
        Ok(Self{name,rules})
    }
}

impl Debug for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{",self.name)?;
        let mut i = self.rules.iter().peekable();
        for r in self.rules.iter() {
            write!(f,"{:?}",r)?;
            i.next();
            if i.peek().is_some() {
                write!(f,",")?;
            }
        }
        write!(f,"}}")?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_workflow_validate() {
        let wf = "ex{x>10:one,m<20:two,a>30:R,A}".parse::<Workflow>().expect("Ops");
        let part = Part{ x: 10, m: 20, a: 20, s: 0 };

        println!("{:?}", wf.validate(part));
        assert_eq!(
            format!("{:?}", Some(Action::Accept)),
            format!("{:?}", wf.validate(part))
        );
    }

    #[test]
        fn test_worflow_parse() {
            let inp = std::fs::read_to_string("src/bin/day19/sample.txt")
                .expect("cannot load sample.txt");
            let data = inp.split("\n\n").next().unwrap().lines();

            for inp in data {
                let wf = inp.parse::<Workflow>().expect("Workflow error!");
                println!("{:?}",wf);
                assert_eq!(format!("{:?}",wf),inp)
            }
        }
}
