use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug,PartialEq)]
pub(crate) struct Universe {
    pub(crate) width: usize,
    pub(crate) length: usize,
    pub(crate) clusters: Vec<Galaxy>,
    pub(crate) x_gap: Vec<usize>,
    pub(crate) y_gap: Vec<usize>
}

impl Universe {
    pub(crate) fn expand(&self, multiplier: usize) -> Vec<Galaxy> {
        let mut clusters = self.clusters.clone();
        let expand = if multiplier > 1 { multiplier - 1 } else { 1 };

        self.x_gap.iter()
            .enumerate()
            .filter(|&(_, count)| 0.eq(count))
            .map(|(x, _)| x)
            .enumerate()
            .for_each(|(i, x)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.0.gt(&(x + i * expand)))
                    .for_each(|g| {
                        g.shift_by((expand, 0));
                    });
            });

        self.y_gap.iter()
            .enumerate()
            .filter(|&(_, count)| 0.eq(count))
            .map(|(y, _)| y)
            .enumerate()
            .for_each(|(i, y)| {
                clusters.iter_mut()
                    .filter(|g| g.pos.1.gt(&(y + i * expand)))
                    .for_each(|g| g.shift_by((0, expand)));
            });

        clusters
    }
}

impl FromStr for Universe {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().peekable();

        let width = lines.peek().unwrap().len();
        let length = input.len()/width;
        let mut x_gap = vec![0; width];
        let mut y_gap = vec![0; length];
        let mut clusters = vec![];

        lines
            .enumerate()
            .for_each(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|&(x, c)| x_gap[x] += '#'.eq(&c) as usize)
                    .filter(|(_, c)| '#'.eq(c))
                    .inspect(|_| y_gap[y] += 1)
                    .map(|(x, _)| Galaxy { pos: (x, y) })
                    .collect_into(&mut clusters);
            });

        Ok( Universe {
            width, length,
            clusters, x_gap, y_gap
        })
    }
}
