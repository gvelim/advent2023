use crate::part::{Part, Unit};
use std::{fmt::Debug, num::ParseIntError, ops::Range, rc::Rc, str::FromStr};

#[derive(Clone, Copy)]
pub(crate) enum PartVar {
    X = 0,
    M,
    A,
    S,
}

impl Debug for PartVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartVar::X => write!(f, "x"),
            PartVar::M => write!(f, "m"),
            PartVar::A => write!(f, "a"),
            PartVar::S => write!(f, "s"),
        }
    }
}

enum Operant {
    GT,
    LT,
}

impl Debug for Operant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operant::GT => write!(f, ">"),
            Operant::LT => write!(f, "<"),
        }
    }
}

pub(crate) struct Condition {
    var: PartVar,
    operant: Operant,
    value: Unit,
}

impl Condition {
    pub(crate) fn part(&self) -> PartVar {
        self.var
    }
    fn validate(&self, part: Part) -> bool {
        match (&self.var, &self.operant) {
            (PartVar::X, Operant::GT) => part.x > self.value,
            (PartVar::X, Operant::LT) => part.x < self.value,
            (PartVar::M, Operant::GT) => part.m > self.value,
            (PartVar::M, Operant::LT) => part.m < self.value,
            (PartVar::S, Operant::GT) => part.s > self.value,
            (PartVar::S, Operant::LT) => part.s < self.value,
            (PartVar::A, Operant::GT) => part.a > self.value,
            (PartVar::A, Operant::LT) => part.a < self.value,
        }
    }
}

impl FromStr for Condition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Condition {
            var: match &s[..1] {
                "x" => PartVar::X,
                "m" => PartVar::M,
                "a" => PartVar::A,
                "s" => PartVar::S,
                _ => panic!("Condition::operant::from_str(): invalid part variable"),
            },
            operant: match &s[1..2] {
                ">" => Operant::GT,
                "<" => Operant::LT,
                _ => panic!("Condition::operant::from_str(): invalid operand"),
            },
            value: Unit::from_str(&s[2..]).expect("Condition::value::from_str(): invalid number"),
        })
    }
}

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}", self.var, self.operant, self.value)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum Action {
    WorkFlow(Rc<str>),
    Accept,
    Reject,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            wf => Self::WorkFlow(wf.into()),
        })
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WorkFlow(s) => write!(f, "{}", s),
            Action::Accept => write!(f, "A"),
            Action::Reject => write!(f, "R"),
        }
    }
}

pub(crate) enum Rule {
    // each rule specifies a condition and where to send the part if the condition is true
    // The last rule in each workflow has no condition and always applies if reached.
    ConAct(Condition, Action),
    Act(Action),
}

impl Rule {
    pub(crate) fn validate(&self, part: Part) -> Option<Action> {
        match self {
            Rule::ConAct(c, a) if c.validate(part) => Some(a.clone()),
            Rule::Act(a) => Some(a.clone()),
            _ => None
        }
    }

    pub(crate) fn validate_range(
        &self,
        rng: &Range<Unit>,
    ) -> Option<(Range<Unit>, Option<Rc<str>>)> {
        match self {
            Rule::ConAct(c, a) => {
                if rng.contains(&c.value) {
                    let r = match c.operant {
                        Operant::GT => {
                            if a == &Action::Reject {
                                rng.start..c.value
                            } else {
                                c.value..rng.end
                            }
                        }
                        Operant::LT => {
                            if a == &Action::Reject {
                                c.value..rng.end
                            } else {
                                rng.start..c.value
                            }
                        }
                    };
                    Some((
                        r,
                        if let Action::WorkFlow(w) = a {
                            Some(w.clone())
                        } else {
                            None
                        },
                    ))
                } else {
                    None
                }
            }
            Rule::Act(Action::WorkFlow(wf)) => Some((rng.clone(), Some(wf.clone()))),
            Rule::Act(Action::Accept) => Some((rng.clone(), None)),
            Rule::Act(Action::Reject) => None,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // x>10:one, m<20:two, a>30:R, A
        let mut s = s.split(':');
        let o = match (s.next(), s.next()) {
            (Some(s), None) => Self::Act(
                s.parse::<Action>()
                    .expect("Rule::Act::Action::from_str failed"),
            ),
            (Some(op), Some(res)) => Self::ConAct(
                op.parse::<Condition>()
                    .expect("Rule::ConAct::Condition::from_str failed"),
                res.parse::<Action>()
                    .expect("Rule::ConAct::Action::from_str failed"),
            ),
            _ => return Err(()),
        };
        Ok(o)
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::ConAct(c, r) => write!(f, "{:?}:{:?}", c, r),
            Rule::Act(r) => write!(f, "{:?}", r),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::workflow::Workflow;

    #[test]
    fn test_rule_check_range() {
        let data = [
            // accept cases / in-bound / conditional
            "x>5:A".parse::<Rule>().unwrap(),
            "m<35:A".parse::<Rule>().unwrap(),
            // reject cases / in-bound / conditional
            "a>30:R".parse::<Rule>().unwrap(),
            "a<10:R".parse::<Rule>().unwrap(),
            // accept cases / off-bound / conditional
            "a<5:A".parse::<Rule>().unwrap(),
            "m>35:A".parse::<Rule>().unwrap(),
            // wf cases / in-bound / conditional
            "m>15:qzz".parse::<Rule>().unwrap(),
            "m<25:hjg".parse::<Rule>().unwrap(),
            // wf cases / off-bound / conditional
            "m<10:qzz".parse::<Rule>().unwrap(),
            "m>30:hjg".parse::<Rule>().unwrap(),
            // non-conditional cases
            "qzz".parse::<Rule>().unwrap(),
            "A".parse::<Rule>().unwrap(),
            "R".parse::<Rule>().unwrap(),
        ];
        let mut rng = 0..40;
        for r in data {
            print!("rule: {:?}, range:{:?} ==> ", r, rng);
            match r.validate_range(&rng) {
                Some((r, wf)) => {
                    rng = r;
                    println!("{:?}", (&rng, wf));
                }
                None => println!("Rejected"),
            }
        }
    }

    #[test]
    fn test_rule_validate() {
        let mut res = [
            Some(Action::WorkFlow("one".into())),
            None,
            Some(Action::WorkFlow("two".into())),
            None,
            Some(Action::Reject),
            None,
            Some(Action::Accept),
        ]
        .into_iter();
        let wf = "ex{x>10:one,x<10:one,m<20:two,m>20:two,a<30:R,a>30:R,A}"
            .parse::<Workflow>()
            .expect("Ops");
        let part = Part {
            x: 11,
            m: 0,
            a: 20,
            s: 0,
        };

        wf.iter().for_each(|rule| {
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
        inp.lines().for_each(|s| {
            let r = s.parse::<Rule>().expect("Rule::parse() error!");
            println!("{:?}", r);
            assert_eq!(&format!("{:?}", r), &s)
        })
    }
}
