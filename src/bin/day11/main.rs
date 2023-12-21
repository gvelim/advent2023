#![feature(iter_collect_into)]
mod universe;
mod galaxy;

use std::str::FromStr;
use crate::universe::Universe;

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        assert_eq!( universe.get_gap_x().collect::<Vec<_>>(), [2,5,8usize]);
        assert_eq!( universe.get_gap_y().collect::<Vec<_>>(), [3,7usize] );
        universe.expand_x();
        universe.expand_y();
        assert_eq!( universe.get_gap_x().collect::<Vec<_>>(), [2,3,6,7,10,11usize] );
        assert_eq!( universe.get_gap_y().collect::<Vec<_>>(), [3,4,8,9usize] );
        println!("{:?}", universe);
    }
    #[test]
    fn test_universe_gaps() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        assert_eq!(
            universe.get_gap_x().collect::<Vec<_>>(),
            [2,5,8usize]
        );
        assert_eq!(
            universe.get_gap_y().collect::<Vec<_>>(),
            [3,7usize]
        );
    }
    #[test]
    fn test_parse_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let universe = input.parse::<Universe>().expect("Failed to parse Universe!");
        
        println!("{:?}",universe);
    }
}