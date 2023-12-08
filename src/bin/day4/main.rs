mod card;
mod numbers;

use crate::{card::Card, numbers::Numbers};

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("Ops!");
    let sum = Rounds::parse_rounds(input.as_str())
        .map(|(card, numbers)| {
            card.winning_numbers(&numbers).len()
        })
        .filter(|size| size > &0)
        .map(|win_nums| 2_u32.pow((win_nums-1) as u32))
        .sum::<u32>();

    println!("Sum: {sum}");
}

struct Rounds;
impl Rounds {
    fn parse_rounds(input: &str) -> impl Iterator<Item=(Card, Numbers)> + '_ {
        input.lines()
            .map(|line| {
                let mut split = line.split("|");
                let mut card = split.next().unwrap().trim().parse::<Card>().expect("Ops");
                let numbers = card.elf_nums;
                card.elf_nums = split.next().unwrap().trim().parse::<Numbers>().ok().expect("win_nums Ops!");
                (card,numbers)
            } )
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
        Rounds::parse_rounds(INPUT)
            .for_each(|card| {
                print!("{:?}\n", card )
            });
    }
    #[test]
    fn test_match_numbers() {

        let sum = Rounds::parse_rounds(INPUT)
            .map(|(card, numbers)| {
                print!("{:?} - Winning Nums = {:?}",card,numbers);
                let win_nums = card.winning_numbers(&numbers);
                println!(" -->  {:?}",win_nums);
                win_nums.len()
            })
            .filter(|size| size > &0)
            .map(|win_nums| 2_u32.pow((win_nums-1) as u32))
            .inspect(|score| println!(" --> Score: {score}"))
            .sum::<u32>();

        println!("Sum: {sum}");
        assert_eq!(sum, 13)
    }
    
}
