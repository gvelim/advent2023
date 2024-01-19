use std::rc::Rc;
use std::str::FromStr;

type FocalLength = usize;
#[derive(Debug,PartialEq)]
pub(crate) enum Operation {
    Remove(Rc<str>),
    Store(Rc<str>,FocalLength)
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(['=','-']);

        Ok(match (parts.next(),parts.next()) {
            (Some(label), Some("")) => Operation::Remove(
                label.into()
            ),
            (Some(label), Some(focal_length)) => Operation::Store(
                label.into(),
                usize::from_str(focal_length).expect("Ops")
            ),
            (_, _) => unreachable!()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Operation::{Store,Remove};
    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-";
    #[test]
    fn test_parse_operation() {
        let ops = INPUT.split(',');
        let cmd = ops
            .map(|op| op.parse::<Operation>().expect("Ops"))
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
