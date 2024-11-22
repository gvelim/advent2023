mod card;
mod numbers;

use std::collections::HashMap;
use crate::card::Rounds;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("src/bin/day4/input.txt").expect("Ops!");

    let t = Instant::now();
    let part1 = Rounds::parse_rounds(input.as_str())
        .map(|(card, numbers)| card.winning_numbers(&numbers).count())
        .filter(|&size| size > 0)
        .map(|size| 2_u32.pow((size - 1) as u32))
        .sum::<u32>();

    println!("Part 1 Sum: {part1} - {:?}", t.elapsed());

    let t = Instant::now();
    let mut part2 = Rounds::parse_rounds(input.as_str())
        .map(|(card,_)| (card.id,1))
        .collect::<HashMap<u32,u32>>();

    let part2_sum = Rounds::parse_rounds(input.as_str())
        .map(|(card, numbers)| {
            let winning_numbers = card.winning_numbers(&numbers).count() as u32;
            (card,winning_numbers)
        })
        .map(|(card, wins)| {
            let card_copies = *part2.get(&card.id).unwrap();
            (card.id+1 ..= card.id + wins)
                .for_each(|next_card| {
                    part2.entry(next_card).and_modify( |next_card_copies| *next_card_copies += card_copies);
            });
            card_copies
        })
        .sum::<u32>();

    println!("Part 2 Sum: {part2_sum} - {:?}", t.elapsed());
}
