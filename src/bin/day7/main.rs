mod hand;

use crate::hand::Hand;
use std::time::Instant;

static CAMEL_ORDER_PART1: [char; 13] = [ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];
static CAMEL_ORDER_PART2: [char; 13] = [ 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A' ];

fn main() {
    let input = std::fs::read_to_string("./src/bin/day7/input.txt").expect("Ops!");

    let run_part = |camel_order, joker| {
        let mut hands = input.lines()
            .map(|line|{
                let mut split = line.split_ascii_whitespace();
                (
                    Hand::parse(split.next().expect("Ops!"), camel_order, joker ),
                    split.next().unwrap().parse::<u32>().expect("Ops!")
                )
            })
            .collect::<Vec<_>>();

        hands.sort();
        hands.iter()
            .enumerate()
            // .inspect(|(i,(h,bid))| print!("Rank {i} - {:?} {bid} => ",(&h.layout,&h.ord_layout,&h.hands_type)))
            .map(|(i,(_,bid))| (i as u32+1) * *bid )
            // .inspect(|ht| println!("{:?}",ht))
            .sum::<u32>()
    };

    let t = Instant::now();
    println!("Part 1 - Total Wins: {:?} - {:?}", run_part(CAMEL_ORDER_PART1, None), t.elapsed());
    let t = Instant::now();
    println!("Part 2 - Total Wins: {:?} - {:?}", run_part(CAMEL_ORDER_PART2, Some('J')), t.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hand::HandType;

    #[test]
    fn test_card_ordering_joker() {
        let (input, _) = parse_input(INPUT);
        let mut hands = input
            .into_iter()
            .map(|line| Hand::parse(line,CAMEL_ORDER_PART2,Some('J')))
            .collect::<Vec<_>>();

        hands.sort();
        assert_eq!(
            vec!["32T3K", "KK677", "JJ958", "T55J5", "QQQJA", "KTJJT"],
            order(&hands)
        )
    }
    #[test]
    fn test_card_ordering() {
        let (input, _) = parse_input(INPUT);
        let mut hands = input
            .into_iter()
            .map(|line| Hand::parse(line,CAMEL_ORDER_PART1,None))
            .collect::<Vec<_>>();

        hands.sort();
        assert_eq!(
            vec!["32T3K", "JJ958", "KTJJT", "KK677", "T55J5", "QQQJA"],
            order(&hands)
        )
    }
    #[test]
    fn test_parse_joker_classify() {
        use HandType as H;

        let (input, _) = parse_input(INPUT);
        let hands = input
            .into_iter()
            .map(|line| Hand::parse(line,CAMEL_ORDER_PART2,Some('J')))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![H::OnePair, H::FourOfAKind, H::TwoPair, H::FourOfAKind, H::FourOfAKind, H::ThreeOfAKind],
            classify(&hands)
        )
    }

    #[test]
    fn test_parse_card_classify() {
        use HandType as H;

        let (input, _) = parse_input(INPUT);
        let hands = input
            .into_iter()
            .map(|line| Hand::parse(line,CAMEL_ORDER_PART1,None))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![H::OnePair, H::ThreeOfAKind, H::TwoPair, H::TwoPair, H::ThreeOfAKind, H::OnePair],
            classify(&hands)
        )
    }

    fn order(hands: &[Hand]) -> Vec<&str> {
        hands.iter()
            .enumerate()
            .inspect(|(i,h)| print!("Rank {i} - {:?} => ",(&h.layout,&h.ord_layout,&h.hands_type)))
            .map(|(_,h)| h.layout.as_str())
            .inspect(|h| println!("{:?}", h))
            .collect::<Vec<&str>>()
    }
    fn classify(hands: &[Hand]) -> Vec<HandType> {
        hands.iter()
            .inspect(|h| print!("{:?} => ",(&h.layout,&h.ord_layout)))
            .map(|h| h.hands_type)
            .inspect(|ht| println!("{:?}",ht))
            .collect::<Vec<HandType>>()
    }

    fn parse_input(input: &str) -> (Vec<&str>,Vec<u32>) {
        input.lines()
            .map(|line|{
                let mut split = line.split_ascii_whitespace();
                (split.next().unwrap(),split.next().unwrap().parse::<u32>().expect("Ops!"))
            })
            .unzip()
    }

    static INPUT: &str= "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\nJJ958 123";
}

