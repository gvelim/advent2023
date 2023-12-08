use super::numbers::Numbers;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Card {
    pub(crate) id: u32,
    pub(crate) elf_nums: Numbers
}

impl Card {
    pub(crate) fn winning_numbers<'a>(&'a self, win_nums: &'a Numbers) -> impl Iterator<Item=&u32> + 'a {
        self.elf_nums.0.intersection(&win_nums.0)
    }
}
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split([':','|']);

        Ok(Card {
            id : u32::from_str(split.next().unwrap().trim().split(' ').last().unwrap()).expect("id:Ops!"),
            elf_nums: split.next().unwrap().trim().parse::<Numbers>().ok().expect("elf_nums Ops!"),
        })
    }
}


pub(crate) struct Rounds;
impl Rounds {
    pub(crate) fn parse_rounds(input: &str) -> impl Iterator<Item=(Card, Numbers)> + '_ {
        input.lines()
            .map(|line| {
                let mut split = line.split("|");
                let mut card = split.next().unwrap().trim().parse::<Card>().expect("Card:Ops");
                let numbers = card.elf_nums;
                card.elf_nums = split.next().unwrap().trim().parse::<Numbers>().ok().expect("elf_nums Ops!");
                (card,numbers)
            } )
    }
}
