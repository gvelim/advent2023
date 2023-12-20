#![feature(iter_collect_into)]

use std::str::FromStr;

fn main() {

}

#[derive(Debug)]
struct Galaxy {
    pos: (usize,usize)
}

impl Galaxy {
    fn shift_by(&mut self, delta: (usize,usize)) {
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }
    fn distance_to(&self, dst: Galaxy) -> usize {
        todo!()
    }
}

#[derive(Debug)]
struct Universe {
    width: usize,
    length: usize,
    galaxies: Vec<Galaxy>,
    x_gap: Vec<u8>,
    y_gap: Vec<u8>,
}

impl Universe {
    fn expand(&mut self) {
        todo!()
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
        let mut galaxies = vec![];

        lines
            .enumerate()
            .for_each(|(y, line)| {
                line.chars()
                    .enumerate()
                    .inspect(|&(x, c)| x_gap[x] += '#'.eq(&c) as u8)
                    .filter(|(_, c)| '#'.eq(c))
                    .inspect(|_| y_gap[y] += 1)
                    .map(|(x, _)| Galaxy { pos: (x, y) })
                    .collect_into(&mut galaxies);
            });

        Ok( Universe { 
            width, length, galaxies, x_gap, y_gap
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_parse_universe() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let universe = input.parse::<Universe>().expect("Failed to parse Universe!");
        
        println!("{:?}",universe);
    }
}