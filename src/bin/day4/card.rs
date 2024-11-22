use super::numbers::{Numbers, NumbersErrors as NE};
use std::{fmt::Display, str::FromStr};
use crate::card::CardError::{InvalidNumericValue, MalformedCardNumbers};

#[derive(Debug)]
pub(crate) struct Card {
    pub(crate) id: u32,
    pub(crate) elf_nums: Numbers
}

impl Card {
    pub(crate) fn winning_numbers<'a>(&'a self, win_nums: &'a Numbers) -> impl Iterator<Item=&'a u32> {
        self.elf_nums.0.intersection(&win_nums.0)
    }
}

#[derive(Debug,PartialEq)]
pub enum CardError {
    MalformedCardRecord,
    MalformedCardNumbers,
    MalformedCard,
    InvalidNumericValue
}

impl Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardError::MalformedCardRecord => write!(f,"Malformed card record; card record is missing"),
            CardError::InvalidNumericValue => write!(f, "Invalid numeric found"),
            CardError::MalformedCardNumbers => write!(f,"Malformed numbers record; numbers are missing"),
            CardError::MalformedCard => write!(f,"Malformed record; record is missing separator")
        }
    }
}

/// this will reduce the need for using map_err() on parse::<Numbers>()
impl From<NE> for CardError {
    fn from(err: NE) -> Self {
        match err {
            NE::InvalidDigit => InvalidNumericValue,
            NE::EmptyInput => MalformedCardNumbers,
            NE::Unknown => MalformedCardNumbers
        }
    }
}

impl FromStr for Card {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CardError as CE;

        if s.split(':').count() != 2 {
            return Err(CE::MalformedCard)
        }

        let mut split = s.split([':']);

        Ok(Card {
            id : u32::from_str(
                    split
                        .next()
                        .ok_or(CE::MalformedCardRecord)?
                        .split_ascii_whitespace()
                        .last()
                        .ok_or(CE::MalformedCardRecord)?
                ).map_err(|_| CE::MalformedCardRecord)?,
            elf_nums: split
                .next()
                .ok_or(CE::MalformedCardNumbers)?
                .parse::<Numbers>()?
        })
    }
}


pub(crate) struct Rounds;
impl Rounds {
    pub(crate) fn parse_rounds(input: &str) -> impl Iterator<Item=(Card, Numbers)> + '_ {
        input.lines()
            .map(|line| {
                let mut split = line.split('|');
                let mut card = split.next().unwrap().parse::<Card>().expect("Card:Ops");
                let numbers = card.elf_nums;
                card.elf_nums = split.next().unwrap().parse::<Numbers>().expect("elf_nums Ops!");
                (card,numbers)
            } )
    }
}



#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;
    use CardError as CE;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parsing_errors() {
        let dataset = [
            ("Card 2: 13 32 20 1a 61", CE::InvalidNumericValue),
            (" : 13 32 20 16 61", CE::MalformedCardRecord),
            ("Card : 13 32 20 16 61", CE::MalformedCardRecord),
            ("13 32 20 16 61 61", CE::MalformedCard),
            ("Card 2", CE::MalformedCard),
            ("Card 2: ", CE::MalformedCardNumbers),
        ];

        for (test, err) in dataset {
            match test.parse::<Card>() {
                Ok(_) => panic!("Test {:?} should not succeed", test),
                Err(e) => {
                    println!("Error: [{}] expecting [{}] in {:?}",e,err,test);
                    assert_eq!(e,err);
                },
            }
        }
    }

    #[test]
    fn test_parsing_of_numbers() {
        Rounds::parse_rounds(INPUT)
            .for_each(|card| {
                println!("{:?}", card )
            });
    }

    #[test]
    fn test_part1() {

        let sum = Rounds::parse_rounds(INPUT)
            .map(|(card, numbers)| {
                print!("{:?} - Winning Nums = {:?}",card,numbers);
                let win_nums = card.winning_numbers(&numbers).count();
                println!(" -->  {:?}",win_nums);
                win_nums
            })
            .filter(|size| size > &0)
            .map(|win_nums| 2_u32.pow((win_nums-1) as u32))
            .inspect(|score| println!(" --> Score: {score}"))
            .sum::<u32>();

        println!("Sum: {sum}");
        assert_eq!(sum, 13)
    }

    #[test]
    fn test_part2() {
        let mut part2 = Rounds::parse_rounds(INPUT)
            .map(|(card,_)| (card.id,1))
            .collect::<HashMap<u32,u32>>();

        let part2_sum = Rounds::parse_rounds(INPUT)
            .map(|(card, numbers)| {
                let winning_numbers = card.winning_numbers(&numbers).count() as u32;
                (card,winning_numbers)
            })
            .inspect(|d| print!("{:?}",d))
            .map(|(card, wins)| {
                let copies = *part2.get(&card.id).unwrap();
                (card.id+1 ..=card.id + wins)
                    .all(|next_card|
                        part2.get_mut(&next_card).map(|d| *d += copies ).is_some()
                );
                copies
            })
            .inspect(|d| println!(" --> Copies: {:?}",d))
            .sum::<u32>();

            println!("Part2 Sum: {part2_sum}");
            assert_eq!(part2_sum, 30)
    }
}
