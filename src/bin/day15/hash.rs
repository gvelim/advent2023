use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Hash(u8);

impl From<Hash> for usize {
    fn from(value: Hash) -> Self {
        value.0 as usize
    }
}

impl FromStr for Hash {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hash(
            s.bytes()
                .fold(0u16, |acc, b|
                    ((acc + b as u16) * 17) % 256
                ) as u8
        ))
    }
}

mod test {
    use crate::Hash;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_sum() {
        let split = INPUT.split(',');

        let sum = split.into_iter()
            .inspect(|s| print!("{:?} -> ",s))
            .map(|hash| -> usize {
                hash.parse::<Hash>().expect("Ops").into()
            })
            .inspect(|h| println!("{:?}",h))
            .sum::<usize>();

        assert_eq!(sum,1320)
    }
    #[test]
    fn test_hash_parsing() {
        let s = "HASH";
        let hash = s.parse::<Hash>().expect("ss");
        println!("{:?} = {:?}",s, hash.0);
        assert_eq!(52,hash.0);
    }
}