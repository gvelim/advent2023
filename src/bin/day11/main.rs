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