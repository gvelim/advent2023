#![feature(iter_collect_into)]
#![feature(isqrt)]

mod universe;
mod galaxy;

use crate::universe::Universe;
use crate::galaxy::Galaxy;

fn main() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_galaxy_distance() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        universe.expand_x();
        universe.expand_y();
        let cluster = universe.clusters;

        assert_eq!(9, cluster[4].distance_to(&cluster[8]));
        assert_eq!(15, cluster[0].distance_to(&cluster[6]));
        assert_eq!(17, cluster[2].distance_to(&cluster[5]));
        assert_eq!(5, cluster[7].distance_to(&cluster[8]));
    }
    #[test]
    fn test_expand_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        universe.expand_x();
        universe.expand_y();
        assert_eq!(
            universe,
            Universe {
                width: 13,
                length: 12,
                clusters: vec![
                    Galaxy { pos: (4, 0) }, Galaxy { pos: (9, 1) },
                    Galaxy { pos: (0, 2) }, Galaxy { pos: (8, 5) },
                    Galaxy { pos: (1, 6) }, Galaxy { pos: (12, 7) },
                    Galaxy { pos: (9, 10) }, Galaxy { pos: (0, 11) },
                    Galaxy { pos: (5, 11) }
                ],
                x_gap: vec![2, 1, 0, 0, 1, 1, 0, 0, 1, 2, 0, 0, 1],
                y_gap: vec![1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 2]
            }
        );
    }
    #[test]
    fn test_parse_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");

        assert_eq!(
            input.parse::<Universe>().expect("Failed to parse Universe!"),
            Universe {
                width: 10,
                length: 10,
                clusters: vec![
                    Galaxy { pos: (3, 0) }, Galaxy { pos: (7, 1) }, Galaxy { pos: (0, 2) },
                    Galaxy { pos: (6, 4) }, Galaxy { pos: (1, 5) }, Galaxy { pos: (9, 6) },
                    Galaxy { pos: (7, 8) }, Galaxy { pos: (0, 9) }, Galaxy { pos: (4, 9) }
                ],
                x_gap: vec![2, 1, 0, 1, 1, 0, 1, 2, 0, 1],
                y_gap: vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 2]
            }
        );

    }
}