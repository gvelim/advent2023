use std::rc::Rc;
use std::str::FromStr;

use crate::hash::{Hash, HashLen};

pub(crate) type FocalLength = usize;
pub(crate) type Label = Rc<str>;

#[derive(Debug,PartialEq)]
pub(crate) enum Instruction {
    Remove(Label),
    Store(Label,FocalLength)
}

impl Instruction {
    pub(crate) fn hash(&self) -> Hash {
        self.label().hash_algo()
    }
    pub(crate) fn label(&self) -> &Label {
        match self {
            Instruction::Remove(l) => l,
            Instruction::Store(l, _) => l,
        }
    }

}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(['=','-']);

        match (parts.next(),parts.next()) {
            (Some(label), Some("")) => Ok(Instruction::Remove(
                label.into()
            )),
            (Some(label), Some(focal_length)) => Ok(Instruction::Store(
                label.into(),
                usize::from_str(focal_length).expect("Ops")
            )),
            (Some(a), b) => Err(format!("Error: potentially parsed a line break ({:?},{:?})",a,b)),
            (a, b) => Err(format!("Error: couldn't read label ({:?},{:?})",a,b)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Instruction::{Store, Remove};

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-";

    #[test]
    fn test_parse_operation() {
        let ops = INPUT.split(',');
        let cmd = ops
            .map(|op| op.parse::<Instruction>().expect("Ops"))
            .collect::<Rc<[_]>>();

        println!("{:?}",cmd);
        assert_eq!(
            cmd,
            [
                Store("rn".into(), 1),
                Remove("cm".into()),
                Store("qp".into(), 3),
                Store("cm".into(), 2),
                Remove("qp".into())
            ].into()
        )
    }
}
