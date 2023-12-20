use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug)]
pub(crate) struct Universe {
    width: usize,
    length: usize,
    clusters: Vec<Vec<Galaxy>>,
    x_gap: Vec<u8>,
    y_gap: Vec<u8>,
}

impl Universe {
    fn expand(&mut self) {
        todo!()
    }
    pub(crate) fn get_gap_x(&self) -> impl Iterator<Item = usize> + '_ {
        self.x_gap.iter()
            .enumerate()
            .filter(|(_,count)| 0u8.eq(count) )
            .map(|(i,_)| i)
    }
    pub(crate) fn get_gap_y(&self) -> impl Iterator<Item = usize> + '_ {
        self.y_gap.iter()
            .enumerate()
            .filter(|(_,count)| 0u8.eq(count) )
            .map(|(i,_)| i)
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
        let mut clusters = vec![vec![]; length];

        lines
            .enumerate()
            .for_each(|(y, line)| {
                let cluster = line.chars()
                    .enumerate()
                    .inspect(|&(x, c)| x_gap[x] += '#'.eq(&c) as u8)
                    .filter(|(_, c)| '#'.eq(c))
                    .inspect(|_| y_gap[y] += 1)
                    .map(|(x, _)| Galaxy { pos: (x, y) })
                    .collect_into(&mut clusters[y]);
            });

        Ok( Universe {
            width, length,
            clusters, x_gap, y_gap
        })
    }
}
