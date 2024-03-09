use std::{collections::HashMap, fmt::Debug, num::ParseIntError, rc::Rc, str::FromStr};

fn main() {

}

type Unit = u16;

struct Part {
    // each part is rated in each of four categories
    x: Unit, // x: Extremely cool looking
    m: Unit, // m: Musical (it makes a noise when you hit it)
    a: Unit, // a: Aerodynamic
    s: Unit  // s: Shiny
}
impl Part {
    fn sum(&self) -> Unit {
        self.x + self.m + self.a + self.s
    }
}

enum Operant { XGT, XLT, MGT, MLT, SGT, SLT, AGT, ALT }
impl Debug for Operant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operant::XGT => write!(f,"x>"),
            Operant::XLT => write!(f,"x<"),
            Operant::MGT => write!(f,"m>"),
            Operant::MLT => write!(f,"m<"),
            Operant::SGT => write!(f,"s>"),
            Operant::SLT => write!(f,"s<"),
            Operant::AGT => write!(f,"a>"),
            Operant::ALT => write!(f,"a<"),
        }
    }
}
struct Condition {
    operant: Operant,
    value: Unit
}
impl FromStr for Condition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Condition {
            operant: match &s[..2] {
                "x>" => Operant::XGT,
                "x<" => Operant::XLT,
                "m>" => Operant::MGT,
                "m<" => Operant::MLT,
                "s>" => Operant::SGT,
                "s<" => Operant::SLT,
                "a>" => Operant::AGT,
                "a<" => Operant::ALT,
                _ => panic!("Condition::operant::from_str(): invalid operand")
            },
            value: Unit::from_str(&s[2..]).expect("Condition::value::from_str(): invalid number")
        })
    }
}
impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}",self.operant,self.value)
    }
}
enum Action {
    WorkFlow(Rc<str>),
    Accept,
    Reject
}
impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            wf => Self::WorkFlow(wf.into())
        })
    }
}
impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WorkFlow(s) => write!(f,"{}",s),
            Action::Accept => write!(f,"A"),
            Action::Reject => write!(f,"R"),
        }
    }
}
enum Rule {
    // each rule specifies a condition and where to send the part if the condition is true
    // The last rule in each workflow has no condition and always applies if reached.
    ConAct(Condition, Action),
    Act(Action)
}
impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // x>10:one, m<20:two, a>30:R, A
        let mut s = s.split(':');
        let o = match (s.next(), s.next()) {
            (Some(s), None) =>
                Self::Act(s.parse::<Action>().expect("Rule::Act::Action::from_str failed")),
            (Some(op), Some(res)) => {
                Self::ConAct(
                    op.parse::<Condition>().expect("Rule::ConAct::Condition::from_str failed"),
                    res.parse::<Action>().expect("Rule::ConAct::Action::from_str failed")
                )
            },
            _ => return Err(()),
        };
        Ok(o)
    }
}
impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::ConAct(c, r) => write!(f, "{:?}:{:?}",c,r),
            Rule::Act(r) => write!(f,"{:?}",r),
        }
    }
}
struct Workflow {
    name: Rc<str>,
    // Each workflow has a name and contains a list of rules
    rules: Rc<[Rule]>
}
impl Workflow {
    fn sort(part: Part) -> Action {
        // The first rule that matches the part being considered is applied immediately,
        // and the part moves on to the destination described by the rule
        Action::Reject
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
enum SortingSystemResult {
    Accept(Part),
    Reject,
}

struct SortingSystem {
    map: HashMap<Rc<str>, Workflow>
}

impl SortingSystem {
    fn sort(part: Part) -> SortingSystemResult {
        // If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns.
        // If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.
        SortingSystemResult::Accept(part)
    }
}




#[cfg(test)]
mod test {
    use crate::{Rule, Workflow};

    #[test]
    fn test_rule_parse() {
        let inp = "x>10:one\nm<20:two\na>30:R\nA";
        inp.lines()
            .for_each(|s| {
                let r = s.parse::<Rule>().expect("Rule::parse() error!");
                println!("{:?}",r);
                assert_eq!(&format!("{:?}",r),&s)
            })
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
