use std::{collections::HashSet, str::FromStr};

fn main() {

}

struct HashNum(HashSet<u32>);
impl FromStr for HashNum {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok( HashNum( input
            .split(' ')
            .filter(|&d| !d.is_empty())
            .map(|num| u32::from_str(num.trim()).expect("Ops!"))
            .collect::<HashSet<u32>>()
        ))
    }
}

#[derive(Debug)]
struct Card {
    id: u32,
    elf_nums: HashSet<u32>,
    win_nums: HashSet<u32>
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split([':','|']);
     
        Ok(Card { 
            id : u32::from_str(split.next().unwrap().trim().split(' ').skip(1).next().unwrap()).expect("id:Ops!"), 
            elf_nums: split.next().unwrap().trim().parse::<HashNum>().ok().expect("elf_nums Ops!").0, 
            win_nums: split.next().unwrap().trim().parse::<HashNum>().ok().expect("win_nums Ops!").0
        })    
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn test_parsing_of_numbers() {
        
        INPUT.lines()
            .map(|line| line.parse::<Card>().expect("Ops") )
            .for_each(|card| {
                print!("{:?}\n", card )
            });   
    }
    
}
