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
    pub(crate) cards: Vec<(char,u8)>,
    joker_pos: Option<usize>
}
impl Hand {
    pub(crate) fn get_type(&self) -> HandType {
        let mut unique_cards = self.cards.len() as u32;
        let mut freq = self.cards[0].1;

        // if we have joker position && and is not a 'JJJJJ' case
        if self.joker_pos.is_some() && unique_cards > 1 {
            unique_cards -= 1;
            freq += self.cards[self.joker_pos.unwrap()].1;
        }

        match unique_cards {
            1 => HandType::FiveOfAKind,
            2 if freq == 4 => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if freq == 3 => HandType::ThreeOfAKind,
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

        let mut joker_pos = None;
        let (cards,ord_layout)= input.chars()
            .fold((HashMap::with_capacity(5),String::with_capacity(5)), |(mut cards, mut ord_layout), card| {
                *cards.entry(card).or_insert(0) += 1;
                ord_layout.push( ord_card[&card]);
                (cards, ord_layout)
            });

        // extract the HashMap onto an array
        let mut cards= cards.into_iter().collect::<Vec<_>>();
        // reverse sort the array by order of card freq
        // hence the most frequent is 1st, then then 2nd least freq, etc
        cards.sort_by_key(|(_,freq)| *freq);
        cards.reverse();

        // if we are dealing with a Joker case
        joker
            .is_some_and(|joker| {
                // find Joker's freq order i.e. 1st, 2nd, etc and store it for later
                // if there is no Joker in the hand, we exit this with None
                // no position == no Joker
                joker_pos = cards.iter().position(|(card,_)| joker.eq(card));
                // if it is 1st and not the only card in the hand; we deal with JJ123 cases
                cards.len() > 1 && joker_pos.eq(&Some(0))
            })
            .then(|| {
                // move to the last place & update its position
                cards.rotate_left(1);
                joker_pos = Some(cards.len()-1);
                Some(())
            });

        let mut hand = Hand {
            layout: String::from(input),
            ord_layout,
            hands_type: HandType::HighCard,
            cards,
            joker_pos
        };
        hand.hands_type = hand.get_type();
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
            .field("Type",&self.hands_type)
            .finish()?;
        f.write_str(", Cards ")?;
        f.debug_map()
            .entries(
                self.cards.iter().map(|(k,v)| (k, v))
            ).finish()?;
        Ok(())
    }
}
