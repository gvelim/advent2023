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
use Instruction as I;

impl Instruction {
    pub(crate) fn hash(&self) -> Hash {
        self.label().hash_algo()
    }
    pub(crate) fn label(&self) -> &Label {
        match self {
            I::Remove(l) => l,
            I::Store(l, _) => l,
        }
    }

}

#[derive(PartialEq)]
pub(crate) enum InstructionError {
    InvalidFocalLength,
    InvalidOperand
}
use InstructionError as IE;

impl Debug for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IE::InvalidFocalLength => write!(f, "Operand '=' is not followed by a number"),
            IE::InvalidOperand => write!(f, "Instruction contains invalid operand ['=','-']"),
        }
    }
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for InstructionError {

}

impl FromStr for Instruction {
    type Err = IE;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(index) = s.chars().position(|c| ['=','-'].contains(&c)) {
            match (s.as_bytes()[index], &s[..index], &s[index+1..]) {
                (b'-', label, _) => Ok(
                    I::Remove(label.into())
                ),
                (b'=', label, fl) => Ok(
                    I::Store( label.into(), usize::from_str(fl).or(Err(IE::InvalidFocalLength))? )
                ),
                _ => Err(IE::InvalidOperand)
            }
        } else {
            Err(IE::InvalidOperand)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
                Ok(I::Store("rn+".into(),1)),
                Ok(I::Remove("cm".into())),
                Err(IE::InvalidOperand),
                Err(IE::InvalidOperand),
                Err(IE::InvalidFocalLength)
            ].into()
        )
    }
}
