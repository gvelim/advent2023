use std::str::FromStr;

fn main() {
    let mut input = std::fs::read_to_string("./src/bin/day15/input.txt").expect("Ops");
    input.pop();
    let split = input.split(',');

    let sum = split.into_iter()
        // .inspect(|s| print!("{:?} -> ",s))
        .map(|hash| hash.parse::<Hash>().expect("Ops"))
        .map(|hash| hash.0 as usize)
        // .inspect(|h| println!("{:?}",h))
        .sum::<usize>();

    println!("Part 1 : Sum of Hashes = {sum}");

}

#[derive(Debug)]
struct Hash(u8);

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
    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_sum() {
        let mut split = INPUT.split(',');

        let sum = split.into_iter()
            .inspect(|s| print!("{:?} -> ",s))
            .map(|hash| hash.parse::<Hash>().expect("Ops"))
            .map(|hash| hash.0 as usize)
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