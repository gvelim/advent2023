use std::ops::Deref;
use std::rc::Rc;

pub(crate) trait HashLen {
    fn hash_algo(&self) -> usize;
}

pub(crate) type Hash = usize;
impl HashLen for &str {
    fn hash_algo(&self) -> Hash {
        self.bytes().fold(0usize, |acc, b| ((acc + b as usize) * 17) % 256 )
    }
}
impl HashLen for Rc<str> {
    fn hash_algo(&self) -> Hash {
        self.deref().hash_algo()
    }
}


mod test {
    use super::*;
    use crate::operation::Label;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_sum() {
        let split = INPUT.split(',');

        let sum = split.into_iter()
            .inspect(|s| print!("{:?} -> ",s))
            .map(|hash| hash.hash_algo() )
            .inspect(|h| println!("{:?}",h))
            .sum::<usize>();

        assert_eq!(sum,1320)
    }
    #[test]
    fn test_hash_parsing() {
        let s: Label = "HASH".into();
        println!("{:?} = {:?}",s, s.hash_algo());
        assert_eq!(52usize,s.hash_algo());
    }
}