#![feature(iter_collect_into)]

mod universe;
mod galaxy;

use crate::universe::Universe;

fn main() {
    let run_part = |universe: &mut Universe, multiplier:usize| -> usize {
        universe.expand(multiplier);
        universe.clusters
            .iter()
            .enumerate()
            .map(|(i, from)| {
                universe.clusters
                    .iter()
                    .skip(i + 1)
                    .map(|to| from.distance_to(to))
                    .sum::<usize>()
            })
            .sum::<usize>()
    };
    let input = std::fs::read_to_string("src/bin/day11/input.txt").expect("Ops!");

    let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");
    let t = std::time::Instant::now();
    println!("Part 1 - Sum of shortest paths: {} - {:?}", run_part(&mut universe,2), t.elapsed());

    let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");
    let t = std::time::Instant::now();
    println!("Part 2 - Sum of shortest paths: {} - {:?}", run_part(&mut universe, 1_000_000), t.elapsed());
}

#[cfg(test)]
mod test {
    use crate::galaxy::Galaxy;
    use super::*;

    #[test]
    fn test_sortest_path() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        universe.expand(100);

        let minsum = universe.clusters
            .iter()
            .enumerate()
            .map(|(i,from)|{
                print!("{:?} -> ",from);
                universe.clusters
                    .iter()
                    .skip(i+1)
                    .inspect(|m| print!("{:?}:",m.pos))
                    .map(|to| from.distance_to(to))
                    .inspect(|m| print!("{m},"))
                    .sum::<usize>()
            })
            .inspect(|m| println!(" = Sum: {m},"))
            .sum::<usize>();

        assert_eq!(minsum,8410);
    }
    #[test]
    fn test_galaxy_distance() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        let cluster = &universe.expand(2).clusters;

        assert_eq!(9, cluster[4].distance_to(&cluster[8]));
        assert_eq!(15, cluster[0].distance_to(&cluster[6]));
        assert_eq!(17, cluster[2].distance_to(&cluster[5]));
        assert_eq!(5, cluster[7].distance_to(&cluster[8]));
    }
    #[test]
    fn test_expand_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        println!("{:?}",universe);
        assert_eq!(
            universe.expand(2),
            &Universe {
                clusters: vec![
                    Galaxy { pos: (4, 0) }, Galaxy { pos: (9, 1) },
                    Galaxy { pos: (0, 2) }, Galaxy { pos: (8, 5) },
                    Galaxy { pos: (1, 6) }, Galaxy { pos: (12, 7) },
                    Galaxy { pos: (9, 10) }, Galaxy { pos: (0, 11) },
                    Galaxy { pos: (5, 11) }
                ]
            }
        );
    }
    #[test]
    fn test_parse_gaps() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        let mut y_gaps = Vec::new();
        let mut x_gaps = Vec::new();

        universe
            .clusters
            .iter()
            .for_each(|g| {
                y_gaps.push(g.pos.1);
                x_gaps.push(g.pos.0);
            });
        x_gaps.sort();

        assert_eq!(
            Universe::extract_gaps(&y_gaps).collect::<Vec<_>>(),
            vec![3..=3, 7..=7]
        );
        assert_eq!(
            Universe::extract_gaps(&x_gaps).collect::<Vec<_>>(),
            vec![2..=2, 5..=5, 8..=8]
        );
    }
    #[test]
    fn test_parse_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");

        assert_eq!(
            input.parse::<Universe>().expect("Failed to parse Universe!"),
            Universe {
                clusters: vec![
                    Galaxy { pos: (3, 0) }, Galaxy { pos: (7, 1) }, Galaxy { pos: (0, 2) },
                    Galaxy { pos: (6, 4) }, Galaxy { pos: (1, 5) }, Galaxy { pos: (9, 6) },
                    Galaxy { pos: (7, 8) }, Galaxy { pos: (0, 9) }, Galaxy { pos: (4, 9) }
                ]
            }
        );

    }
}