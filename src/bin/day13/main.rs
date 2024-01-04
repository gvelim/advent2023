#![feature(iter_collect_into)]
mod valley;
mod pattern;

use crate::valley::Valley;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let mut valley = input.parse::<Valley>().expect("Ops!");

    let t = std::time::Instant::now();
    println!("Part 1 : {:?} - {:?}", valley.summarise_notes(), t.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_sample_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample_p2.txt").expect("Ops!");
        let mut valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_notes(),405);
    }
    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample_p2.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_vertical_mirror_max())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_horizontal_mirror_max())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }
}