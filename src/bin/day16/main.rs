use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

type Position = usize;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day16/input.txt").expect("Ops!");
    let mut cavern = inp.parse::<Cavern>().unwrap_or_default();

    let t = std::time::Instant::now();
    cavern.move_beam(0,D::Right);
    println!("Part 1 : Cavern Energy = {:?} - {:?}",cavern.energy(),t.elapsed());
    assert_eq!(cavern.energy(),6902);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Direction { Up, Left, Down, Right, UpDown, LeftRight }
use Direction as D;

impl Direction {
    fn direction(&mut self, tile: u8) -> &Direction {
        *self = match (tile, *self) {
            (b'/', D::Right) => D::Up,
            (b'/', D::Left) => D::Down,
            (b'/', D::Up) => D::Right,
            (b'/', D::Down) => D::Left,
            (b'\\', D::Right) => D::Down,
            (b'\\', D::Left) => D::Up,
            (b'\\', D::Up) => D::Left,
            (b'\\', D::Down) => D::Right,
            (b'-', D::Up | D::Down) => D::LeftRight,
            (b'|' , D::Right | D::Left) => D::UpDown,
            (_,d) => d
        };
        self
    }
}

#[derive(Default)]
pub(crate) struct Cavern {
    width: usize,
    lines: usize,
    con: Vec<u8>,
    nrg: Vec<u8>,
    tail: HashMap<usize,Direction>
}

impl Cavern {
    fn step(&self, idx: usize, dir:Direction) -> Option<Position> {
        match dir {
            D::Right if idx % self.width < self.width-1 => Some(idx + 1),
            D::Left if idx % self.width > 0 => Some(idx - 1),
            D::Up if (self.width..self.con.len()).contains(&idx) => Some(idx - self.width),
            D::Down if idx < self.con.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
    fn move_beam(&mut self, idx: usize, mut dir:Direction) -> Option<usize> {

        if Some(true) == self.tail.get(&idx).map(|d| dir == *d) { return None }
        if self.con[idx] != b'.' { self.tail.insert(idx,dir); }

        self.nrg[idx] = b'#';

        match *dir.direction(self.con[idx]) {
            D::Right | D::Left | D::Up | D::Down => {
                self.step(idx, dir).and_then(|next| self.move_beam(next, dir))
            },
            D::LeftRight =>
                Some(
                    self.step(idx, D::Left).and_then(|next| self.move_beam(next, D::Left)).unwrap_or(0)
                    + self.step(idx, D::Right).and_then(|next| self.move_beam(next, D::Right)).unwrap_or(0)
                ),
            D::UpDown =>
                Some(
                    self.step(idx, D::Down).and_then(|next| self.move_beam(next, D::Down)).unwrap_or(0)
                    + self.step(idx, D::Up).and_then(|next| self.move_beam(next, D::Up)).unwrap_or(0)
                )
        }
            .map(|count| count + 1)
    }
    fn energy(&self) -> usize {
        self.nrg.iter().filter(|c| b'#'.eq(c)).count()
    }
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cavern {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            con: s.lines()
                .flat_map(|line| line.bytes())
                .collect::<Vec<_>>(),
            nrg: vec![b'.'; s.len()],
            tail: HashMap::new()
        })
    }
}

impl Debug for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Cavern\n")?;
        writeln!(f,"Width:{}, Length:{}", self.width, self.lines)?;

        let mut citer = self.con.iter();
        let mut eiter = self.nrg.iter();
        for _ in 0..self.lines {
            for _ in 0..self.width { write!(f, "{} ", *citer.next().unwrap() as char)? };
            write!(f, "  ")?;
            for _ in 0..self.width { write!(f, "{} ", *eiter.next().unwrap() as char)? };
            writeln!(f)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use crate::{Cavern, D};

    #[test]
    fn test_move_bean() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let mut cavern = inp.parse::<Cavern>().unwrap_or_default();

        println!("{:?}",cavern.move_beam(0,D::Right));
        println!("{:?}",cavern);
        assert_eq!(cavern.energy(),46);
    }

    #[test]
    fn test_parse_cavern() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let cavern = inp.parse::<Cavern>().unwrap_or_default();
        println!("{:?}",cavern);
    }
}