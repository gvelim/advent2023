use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug,PartialEq)]
pub(crate) struct Universe {
    pub(crate) clusters: Vec<Galaxy>
}

impl Universe {
    pub(crate) fn expand(&self, multiplier: usize) -> Vec<Galaxy> {
        let mut clusters = self.clusters.clone();
        let expand = if multiplier > 1 { multiplier - 1 } else { 1 };

        let (mut x_gap, mut y_gap) = (vec![], vec![]);

        clusters.iter().for_each(|g| {
            x_gap.push(g.pos.0);
            y_gap.push(g.pos.1);
        });

        x_gap.sort();

        Universe::extract_gaps(&x_gap)
            .flatten()
            .enumerate()
            .for_each(|(i, x)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.0.gt(&(x + i * expand)))
                    .for_each(|g| {
                        g.shift_by((expand, 0));
                    });
            });

        Universe::extract_gaps(&y_gap)
            .flatten()
            .enumerate()
            .for_each(|(i,y)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.1.gt(&(y + i * expand)))
                    .for_each(|g|
                        g.shift_by((0, expand))
                    );
            });

        clusters
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
