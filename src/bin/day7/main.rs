use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use HandsType::*;

fn main() {

}

static CAMEL_CARD: [char; 13] = [ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq,Copy, Clone)]
enum HandsType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    layout: String,
    hands_type: HandsType,
    cards: HashMap<char,u8>
}
impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut most_common = 0;
        let cards= s.chars()
            .fold(HashMap::new(), |mut cards, card| {
                let c = cards.entry(card).or_insert(0);
                *c += 1;
                most_common = std::cmp::max(most_common, *c);
                cards
            });
        Ok(Hand {
            layout: String::from(s),
            hands_type: match cards.len() {
                1 => FiveOfAKind,
                2 if most_common ==4 => FourOfAKind,
                2 => FullHouse,
                3 if most_common ==3 => ThreeOfAKind,
                3 => TwoPair,
                4 => OnePair,
                _ => HighCard
            },
            cards
        })
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Layout {:?}, ",&self.layout))?;
        f.debug_struct("Hand")
            .field("type",&self.hands_type)
            .finish()?;
        f.write_str(", Cards ")?;
        f.debug_map()
            .entries(
                self.cards.iter().map(|(k,v)| (k, v))
            ).finish()?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str= "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_parse_card_classify() {
        let input= "32T3K\nT55J5\nKK677\nKTJJT\nQQQJA";
        let hands = input
            .lines()
            .map(|line| line.parse::<Hand>().expect("ops"))
            .collect::<Vec<_>>();

        assert_eq!(
            vec![OnePair, ThreeOfAKind, TwoPair, TwoPair, ThreeOfAKind],
            hands.iter()
                .inspect(|h| print!("{:?} => ",h.layout))
                .map(|h| h.hands_type)
                .inspect(|ht| println!("{:?}",ht))
                .collect::<Vec<HandsType>>()
        )
    }

}

