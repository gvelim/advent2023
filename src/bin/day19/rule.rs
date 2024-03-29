use std::{fmt::Debug, num::ParseIntError, rc::Rc, str::FromStr};
use crate::part::{Part, Unit};

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

impl Condition {
    fn validate(&self, part: Part) -> bool {
        match self.operant {
            Operant::XGT => part.x > self.value,
            Operant::XLT => part.x < self.value,
            Operant::MGT => part.m > self.value,
            Operant::MLT => part.m < self.value,
            Operant::SGT => part.s > self.value,
            Operant::SLT => part.s < self.value,
            Operant::AGT => part.a > self.value,
            Operant::ALT => part.a < self.value,
        }
    }
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

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum Action {
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

pub(crate) enum Rule {
    // each rule specifies a condition and where to send the part if the condition is true
    // The last rule in each workflow has no condition and always applies if reached.
    ConAct(Condition, Action),
    Act(Action)
}

impl Rule {
    pub(crate) fn validate(&self, part: Part) -> Option<Action> {
        match self {
            Rule::ConAct(c, a) if c.validate(part) => Some(a.clone()),
            Rule::Act(a) => Some(a.clone()),
            _ => None
        }
    }

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



#[cfg(test)]
mod test {
    use super::*;
    use crate::workflow::Workflow;

    #[test]
    fn test_rule_validate() {
        let mut res = [
            Some(Action::WorkFlow("one".into())),
            None,
            Some(Action::WorkFlow("two".into())),
            None,
            Some(Action::Reject),
            None,
            Some(Action::Accept)
        ]
        .into_iter();
        let wf = "ex{x>10:one,x<10:one,m<20:two,m>20:two,a<30:R,a>30:R,A}".parse::<Workflow>().expect("Ops");
        let part = Part{ x: 11, m: 0, a: 20, s: 0 };

        wf.iter()
            .for_each(|rule|{
                println!("{:?} => {:?} = {:?}", rule, part, rule.validate(part));
                assert_eq!(
                    format!("{:?}", res.next().unwrap()),
                    format!("{:?}", rule.validate(part))
                );
            });
    }

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
}
