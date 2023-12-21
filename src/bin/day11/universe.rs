use std::str::FromStr;
use crate::galaxy::Galaxy;

#[derive(Debug)]
pub(crate) struct Universe {
    width: usize,
    length: usize,
    pub(crate) clusters: Vec<Galaxy>,
    x_gap: Vec<usize>,
    y_gap: Vec<usize>,
}

impl Universe {
    pub(crate) fn expand_x(&mut self) {
        let mut gap = vec![];

        self.x_gap.iter()
            .enumerate()
            .filter(|&(_,count)| 0.eq(count) )
            .map(|(x,_)| x)
            .enumerate()
            .for_each(|(i,x)|{

                self.clusters.iter_mut()
                    .filter(|g| g.pos.0.gt(&(x + i)) )
                    .for_each(|g| g.pos.0 += 1 );

                gap.push(x + i);
            });

        self.width += gap.len();
        gap.into_iter().for_each(|idx| self.x_gap.insert(idx,0))
    }

    pub(crate) fn expand_y(&mut self) {
        let mut gap = vec![];

        self.y_gap.iter()
            .enumerate()
            .filter(|&(_,count)| 0.eq(count) )
            .map(|(y,_)| y)
            .enumerate()
            .for_each(|(i,y)|{

                self.clusters.iter_mut()
                    .filter(|g| g.pos.1.gt(&(y + i)) )
                    .for_each(|g| g.pos.1 += 1 );

                gap.push(y + i);
            });

        self.length += gap.len();
        gap.into_iter().for_each(|idx| self.y_gap.insert(idx,0))
    }

    pub(crate) fn get_gap_x(&self) -> impl Iterator<Item = usize> + DoubleEndedIterator + '_ {
        self.x_gap.iter()
            .enumerate()
            .filter(|&(_,count)| 0.eq(count) )
            .map(|(i,_)| i)
    }
    pub(crate) fn get_gap_y(&self) -> impl Iterator<Item = usize> + DoubleEndedIterator + '_ {
        self.y_gap.iter()
            .enumerate()
            .filter(|&(_,count)| 0.eq(count) )
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
