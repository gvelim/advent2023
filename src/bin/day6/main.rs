#![feature(isqrt)]

mod race;

use crate::race::*;
use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day6/input.txt").unwrap_or_default();
    let races = Race::parse_races(input.as_str());

    let t = Instant::now();
    let product = races
        // .map(|race| race.winning_charge_times().collect::<Vec<_>>() )
        .map(|race|
            (race.find_upper_winning_charge(), race.find_lower_winning_charge())
        )
        // .map(|wins| wins.len() as u64)
        .map(|(ub,lb)| ub-lb+1)
        .product::<u64>();

    println!("Part 1: product = {product} - {:?}",t.elapsed());

    let race = Race::parse_whole_numbers(input.as_str()).expect("");

    let t = Instant::now();
    let lb = race.find_lower_winning_charge();
    let ub = race.find_upper_winning_charge();
    println!("Part 2: Bounds {:?} -> {} - {:?}",(lb,ub), ub-lb+1, t.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "Time:      7  15   30\n\
                          Distance:  9  40  200";
    #[test]
    fn test_find_winning_bounds() {
        let races = Race::parse_races(INPUT);
        assert_eq!(
            races
                .map(|race|
                    (race.find_lower_winning_charge(),race.find_upper_winning_charge())
                )
                .collect::<Vec<_>>(),
            [(2u64,5u64),(4,11),(11,19)]
        )
    }
    #[test]
    fn test_find_winning_bounds_whole_numbers() {
        let race = Race::parse_whole_numbers(INPUT).expect("");
        let bounds = (race.find_lower_winning_charge(), race.find_upper_winning_charge());
        println!("{:?}\nCharge bounds {:?}",&race, bounds);
        assert_eq!(bounds,(14,71516))
    }
    #[test]
    fn test_parse_whole_numbers() {
        assert_eq!(
            Race { duration:71530, record:940200 },
          Race::parse_whole_numbers(INPUT).expect("")
        )
    }
    #[test]
    fn test_ways_to_beat_record() {
        let races = Race::parse_races(INPUT);
        assert_eq!(
            288,
            races.into_iter()
                .inspect(|race| print!("{:?}",race))
                .map(|race| race.winning_charge_times().collect::<Vec<_>>() )
                .inspect(|ways| println!("-> {:?}",ways))
                .map(|ways| ways.len() as u32)
                .product::<u32>()
        )
    }
    #[test]
    fn test_winning_charge_times() {
        let race = Race::parse_races(INPUT).next().unwrap();
        assert_eq!(
            race
                .winning_charge_times()
                .inspect(|dist| print!("{:?},",dist))
                .collect::<Vec<_>>(),
            [(2, 10), (3, 12), (4, 12), (5, 10)]
        )
    }
    #[test]
    fn test_trial_charge_times() {
        let race = Race::parse_races(INPUT).next().unwrap();
        assert_eq!(
            race
                .trial_charge_times()
                .inspect(|dist| print!("{:?},",dist))
                .collect::<Vec<_>>(),
            [(0, 0), (1, 6), (2, 10), (3, 12), (4, 12), (5, 10), (6, 6), (7,0)]
        )
    }
    #[test]
    fn test_parse_races() {
        assert_eq!(
            Race::parse_races(INPUT)
                .inspect(|d| println!("{:?}", d))
                .next()
                .unwrap(),
            (7,9).into()
        )
    }
}