use std::cmp::Ordering;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

static CAMEL_CARD: [char; 13] = [ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq,Copy, Clone)]
pub(crate) enum HandsType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub(crate) struct Hand {
    pub(crate) layout: String,
    pub(crate) hands_type: HandsType,
    pub(crate) ord_layout: String,
    cards: HashMap<char,u8>
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hands_type.cmp(&other.hands_type) {
            Ordering::Equal =>
                self.ord_layout.cmp(&other.ord_layout),
            comparison => comparison
        }
    }
}
impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HandsType::*;

        let ord_card = CAMEL_CARD.iter()
            .zip([ '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E' ])
            .map(|(&i,o)| (i,o) )
            .collect::<HashMap<char,char>>();

        let mut most_common = 0;
        let (cards,ord_layout)= s.chars()
            .fold((HashMap::new(),String::new()), |(mut cards, mut ord_layout), card| {
                let c = cards.entry(card).or_insert(0);
                *c += 1;
                most_common = std::cmp::max(most_common, *c);
                ord_layout.push( ord_card[&card]);
                (cards, ord_layout)
            });
        Ok(Hand {
            layout: String::from(s),
            ord_layout,
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
