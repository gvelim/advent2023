mod hand;

use crate::hand::{HandsType, Hand};

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_ordering() {
        let (input, _) = parse_input(INPUT);
        let mut hands = input
            .into_iter()
            .map(|line| line.parse::<Hand>().expect("ops"))
            .collect::<Vec<_>>();

        hands.sort();
        assert_eq!(
            vec!["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"],
            hands.iter()
                .enumerate()
                .inspect(|(i,h)| print!("Rank {i} - {:?} => ",(&h.layout,&h.ord_layout,&h.hands_type)))
                .map(|(_,h)| h.layout.as_str())
                .inspect(|h| println!("{:?}", h))
                .collect::<Vec<&str>>()
        )
    }
    #[test]
    fn test_parse_card_classify() {
        use HandsType::*;

        let (input, _) = parse_input(INPUT);
        let hands = input
            .into_iter()
            .map(|line| line.parse::<Hand>().expect("ops"))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![OnePair, ThreeOfAKind, TwoPair, TwoPair, ThreeOfAKind],
            hands.iter()
                .inspect(|h| print!("{:?} => ",(&h.layout,&h.ord_layout)))
                .map(|h| h.hands_type)
                .inspect(|ht| println!("{:?}",ht))
                .collect::<Vec<HandsType>>()
        )
    }

    fn parse_input(input: &str) -> (Vec<&str>,Vec<u32>) {
        input.lines()
            .map(|line|{
                let mut split = line.split_ascii_whitespace();
                (split.next().unwrap(),u32::from_str_radix(split.next().unwrap(),10).expect("Ops!"))
            })
            .unzip()
    }

    static INPUT: &str= "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
}

