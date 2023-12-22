use std::ops::RangeInclusive;
use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug,PartialEq)]
pub(crate) struct Universe {
    pub(crate) width: usize,
    pub(crate) length: usize,
    pub(crate) clusters: Vec<Galaxy>,
    pub(crate) x_gap: Vec<RangeInclusive<usize>>,
    pub(crate) y_gap: Vec<RangeInclusive<usize>>
}

impl Universe {
    pub(crate) fn expand(&self, multiplier: usize) -> Vec<Galaxy> {
        let mut clusters = self.clusters.clone();
        let expand = if multiplier > 1 { multiplier - 1 } else { 1 };

        self.x_gap.iter()
            .enumerate()
            .for_each(|(i, x)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.0.gt(&(x.end() + i * expand)))
                    .for_each(|g| {
                        g.shift_by((expand, 0));
                    });
            });

        self.y_gap.iter()
            .enumerate()
            .for_each(|(i, y)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.1.gt(&(y.end() + i * expand)))
                    .for_each(|g|
                        g.shift_by((0, expand))
                    );
            });

        clusters
    }
    pub(crate) fn derive_gaps(seq: &Vec<usize>) -> impl Iterator<Item=RangeInclusive<usize>> + '_ {
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
        let mut lines = input.lines().peekable();

        let width = lines.peek().unwrap().len();
        let length = input.len()/width;
        let mut x_gap = vec![];
        let mut y_gap = vec![];
        let mut clusters = vec![];

        lines
            .enumerate()
            .for_each(|(y, line)| {
                line.chars()
                    .enumerate()
                    // .inspect(|&(x, c)| x_gap[x] += '#'.eq(&c) as usize)
                    .filter(|(_, c)| '#'.eq(c))
                    // .inspect(|_| y_gap[y] += 1)
                    .map(|(x, _)| {
                        x_gap.push(x);
                        y_gap.push(y);
                        Galaxy { pos: (x, y) }
                    })
                    .collect_into(&mut clusters);
            });
        x_gap.sort();

        Ok( Universe {
            width, length,
            clusters,
            x_gap: Universe::derive_gaps(&x_gap).collect::<Vec<RangeInclusive<usize>>>(),
            y_gap: Universe::derive_gaps(&y_gap).collect::<Vec<RangeInclusive<usize>>>()
        })
    }
}
