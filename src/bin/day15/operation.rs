use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
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

#[derive(PartialEq)]
pub(crate) enum IErr {
    InvalidFocalLength,
    InvalidOperand
}

impl Debug for IErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IErr::InvalidFocalLength => write!(f,"Operand '=' is not followed by a number"),
            IErr::InvalidOperand => write!(f,"Instruction contains invalid operand ['=','-']"),
        }
    }
}

impl Display for IErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for IErr {

}

impl FromStr for Instruction {
    type Err = IErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(index) = s.chars().position(|c| ['=','-'].contains(&c)) {
            match (s.as_bytes()[index], &s[..index], &s[index+1..]) {
                (b'-', label, _) => Ok(
                    Instruction::Remove(label.into())
                ),
                (b'=', label, fl) => Ok(
                    Instruction::Store(
                        label.into(),
                        usize::from_str(fl)
                            .or_else(|_| Err(IErr::InvalidFocalLength))?
                    )
                ),
                _ => Err(IErr::InvalidOperand)
            }
        } else {
            Err(IErr::InvalidOperand)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Instruction::{Store, Remove};

    #[test]
    fn test_parse_operation() {
        let ops = "rn+=1,cm-,rn)1,qm,rn=o".split(',');
        let cmd = ops
            .map(|op| op.parse::<Instruction>())
            .collect::<Rc<[_]>>();

        println!("{:?}",cmd);
        assert_eq!(
            cmd,
            [
                Ok(Store("rn+".into(),1)),
                Ok(Remove("cm".into())),
                Err(IErr::InvalidOperand),
                Err(IErr::InvalidOperand),
                Err(IErr::InvalidFocalLength)
            ].into()
        )
    }
}
