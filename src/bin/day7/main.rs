mod hand;

use crate::hand::{HandsType, Hand};

fn main() {
    let input = std::fs::read_to_string("./src/bin/day7/input.txt").expect("Ops!");

    let mut hands = input.lines()
        .map(|line|{
            let mut split = line.split_ascii_whitespace();
            (split.next().unwrap().parse::<Hand>().expect("Ops!"), u32::from_str_radix(split.next().unwrap(),10).expect("Ops!"))
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a,b| a.cmp(&b));
    let total_wins = hands.iter()
        .enumerate()
        .inspect(|(i,(h,bid))| print!("Rank {i} - {:?} {bid} => ",(&h.layout,&h.ord_layout,&h.hands_type)))
        .map(|(i,(h,bid))| (i as u32+1) * *bid )
        .inspect(|ht| println!("{:?}",ht))
        .sum::<u32>();

    println!("Part 1 - Total Wins: {total_wins}");
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

