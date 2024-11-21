use crate::iterator::*;
use std::{num::ParseIntError, rc::Rc, str::FromStr};

pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Rc<[Number]>
}

impl Sequence {
    pub(crate) fn iter_forward(&self) -> FwdIterator {
        FwdIterator::new(&self.history)
    }
    pub(crate) fn iter_backward(&self) -> BkwIterator {
        BkwIterator::new(&self.history)
    }
}

impl FromStr for Sequence {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_ascii_whitespace()
            .map(|s| s.parse::<Number>())
            .collect::<Result<Rc<[_]>,_>>()
        {
            Ok(history) => Ok(Sequence { history }),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_parse_error() {
        use std::num::IntErrorKind;

        let inputs = [
            ("0 3 4 b ' 4", IntErrorKind::InvalidDigit),
            ("0 -99999999999 2 3", IntErrorKind::NegOverflow),
            ("0 999999999999999 2 3", IntErrorKind::PosOverflow)
        ];

        for (test, err) in inputs {
            match test.parse::<Sequence>() {
                Err(e) => {
                    println!("Test {:?}", (test, &e));
                    assert_eq!( e.kind(), &err )
                },
                Ok(res) => unreachable!("Should never receive {:?}",res),
            }
        }
    }

    #[test]
    fn test_parse() {
        use std::rc::Rc;

        let seq = INPUT.lines()
            .map(|line| line.parse::<Sequence>().expect("Ops!"))
            .collect::<Vec<_>>();

        seq.iter().for_each(|s| println!("{:?}",s) );
        assert_eq!(
            seq,
            [
                Sequence { history: Rc::from([0, 3, 6, 9, 12, 15]) },
                Sequence { history: Rc::from([1, 3, 6, 10, 15, 21]) },
                Sequence { history: Rc::from([10, 13, 16, 21, 30, 45]) },
            ]
        )
    }
}
