use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug,PartialEq)]
pub(crate) struct Universe {
    pub(crate) clusters: Vec<Galaxy>
}

impl Universe {
    pub(crate) fn expand(&mut self, multiplier: usize) -> &Self {
        let expand = if multiplier > 1 { multiplier - 1 } else { 1 };

        let (mut x_gap, mut y_gap) = (vec![], vec![]);

        self.clusters.iter().for_each(|g| {
            x_gap.push(g.pos.0);
            y_gap.push(g.pos.1);
        });

        x_gap.sort();

        let mut i = 0;
        Universe::extract_gaps(&x_gap)
            .for_each(|x| {
                let len = x.end() - x.start() + 1;
                self.clusters.iter_mut()
                    .filter(|g| g.pos.0.gt(&(x.end() + i * expand)))
                    .for_each(|g| {
                        g.shift_by((expand * len, 0));
                    });
                i += len;
            });

        i = 0;
        Universe::extract_gaps(&y_gap)
            .for_each(|y| {
                let len = y.end() - y.start() + 1;
                self.clusters.iter_mut()
                    .filter(|g| g.pos.1.gt(&(y.end() + i * expand)))
                    .for_each(|g|
                        g.shift_by((0, expand * len))
                    );
                i += len;
            });
        self
    }
    pub(crate) fn extract_gaps(seq: &Vec<usize>) -> impl Iterator<Item=RangeInclusive<usize>> + '_ {
        seq.windows(2)
            .filter_map(|pair| {
                let [a,b] = pair else { unreachable!() };
                let gap = b - a;
                if gap > 1 {
                    Some(b - gap + 1 ..= *b - 1)
                } else {
                    None
                }
            })
    }
}

impl FromStr for Universe {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut clusters = vec![];

        input
            .lines()
            .enumerate()
            .for_each(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| '#'.eq(c))
                    .map(|(x, _)| {
                        Galaxy { pos: (x, y) }
                    })
                    .collect_into(&mut clusters);
            });

        Ok( Universe { clusters })
    }
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