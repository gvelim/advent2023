use super::numbers::Numbers;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Card {
    pub(crate) id: u32,
    pub(crate) elf_nums: Numbers
}

impl Card {
    pub(crate) fn winning_numbers(&self, win_nums: &Numbers) -> Vec<u32> {
        self.elf_nums.0.intersection(&win_nums.0).copied().collect::<Vec<u32>>()
    }
}
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split([':','|']);

        Ok(Card {
            id : u32::from_str(split.next().unwrap().trim().split(' ').skip(1).next().unwrap()).expect("id:Ops!"),
            elf_nums: split.next().unwrap().trim().parse::<Numbers>().ok().expect("elf_nums Ops!"),
        })
    }
}
