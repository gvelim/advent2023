use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq,Copy, Clone)]
pub(crate) enum HandType {
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
    pub(crate) hands_type: HandType,
    pub(crate) ord_layout: String,
    pub(crate) cards: HashMap<char,u8>,
    most_common: u8
}
impl Hand {
    pub(crate) fn get_type(&self, joker: Option<char>) -> HandType {
        let mut cards = self.cards.len() as u32;
        let mut most_common = self.most_common;

        if joker.is_some() && cards > 1 {
            if let Some(&joker) = self.cards.get(&joker.unwrap()) {
                cards -= 1;
                most_common += joker;
            }
        }

        match cards {
            1 => HandType::FiveOfAKind,
            2 if most_common ==4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if most_common ==3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
    pub(crate) fn parse(input: &str, card_order: [char; 13], joker:Option<char>) -> Hand {

        let ord_card = card_order.iter()
            .zip([ '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E' ])
            .map(|(&i,o)| (i,o) )
            .collect::<HashMap<char,char>>();

        let mut most_common = 0;
        let (cards,ord_layout)= input.chars()
            .fold((HashMap::new(),String::new()), |(mut cards, mut ord_layout), card| {
                let c = cards.entry(card).or_insert(0);
                *c += 1;
                most_common = std::cmp::max(most_common, *c);
                ord_layout.push( ord_card[&card]);
                (cards, ord_layout)
            });

        let mut hand = Hand {
            layout: String::from(input),
            ord_layout,
            most_common,
            hands_type: HandType::HighCard,
            cards
        };

        hand.hands_type = hand.get_type(joker);
        hand
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, _other: &Self) -> bool {
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
        // if let Some(joker) = self.cards.get(&'J') {
        //     println!(" {:?} ",(joker,self.cards.len(),self.get_type(Some('J')), self));
        // }
        match self.hands_type.cmp(&other.hands_type) {
            Ordering::Equal =>
                self.ord_layout.cmp(&other.ord_layout),
            comparison => comparison
        }
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
