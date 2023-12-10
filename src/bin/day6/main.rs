mod race;

use crate::race::*;

fn main() {

}

#[cfg(test)]
mod test {
    use  super::*;

    static INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

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