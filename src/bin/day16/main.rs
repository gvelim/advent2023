use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use Direction as D;
type Position = usize;

fn main() {
    todo!()
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Direction { North=0, West, South, East }

#[derive(Default)]
pub(crate) struct Cavern {
    width: usize,
    lines: usize,
    layout: Vec<u8>,
    hash: HashMap<usize,bool>
}

impl Cavern {
    fn next(&self, idx: usize, dir:Direction) -> Option<Position> {
        match dir {
            D::East if idx % self.width < self.width-1 => Some(idx + 1),
            D::West if idx % self.width > 0 => Some(idx - 1),
            D::North if (self.width..self.layout.len()).contains(&idx) => Some(idx - self.width),
            D::South if idx < self.layout.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
    fn move_beam(&mut self, idx: usize, dir:Direction) -> Option<usize> {
        if idx >= self.layout.len() { return None }
        print!("{idx},");
        if self.hash.insert(idx,true).is_some() {
            return Some(0)
        }

        match (self.layout[idx], dir) {
            (b'.', _) => self.next(idx,dir).and_then(|next| self.move_beam(next, dir)),
            (b'/', D::East) => self.next(idx,D::North).and_then(|next| self.move_beam(next,D::North)),
            (b'/', D::West) => self.next(idx,D::South).and_then(|next| self.move_beam(next,D::South)),
            (b'/', D::North) => self.next(idx,D::East).and_then(|next| self.move_beam(next,D::East)),
            (b'/', D::South) => self.next(idx,D::West).and_then(|next| self.move_beam(next,D::West)),
            (b'\\', D::East) => self.next(idx,D::South).and_then(|next| self.move_beam(next,D::South)),
            (b'\\', D::West) => self.next(idx,D::North).and_then(|next| self.move_beam(next,D::North)),
            (b'\\', D::North) => self.next(idx,D::West).and_then(|next| self.move_beam(next,D::West)),
            (b'\\', D::South) => self.next(idx,D::East).and_then(|next| self.move_beam(next,D::East)),
            (b'-', D::East|D::West) => self.next(idx,dir).and_then(|next| self.move_beam(next,dir)),
            (b'-', D::North|D::South) =>
                Some(
                    self.next(idx,D::West).and_then(|next| self.move_beam(next,D::West)).unwrap_or(0)
                    + self.next(idx,D::East).and_then(|next| self.move_beam(next,D::East)).unwrap_or(0)
                ),
            (b'|' , D::South|D::North) => self.next(idx,dir).and_then(|next| self.move_beam(next,dir)),
            (b'|' , D::East|D::West) =>
                Some(
                    self.next(idx,D::South).and_then(|next| self.move_beam(next,D::South)).unwrap_or(0)
                    + self.next(idx,D::North).and_then(|next| self.move_beam(next,D::North)).unwrap_or(0)
                ),
            _ => unreachable!()
        }.map(|count| count + 1)
    }

}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cavern {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            layout: s.lines()
                .flat_map(|line| line.bytes())
                .collect::<Vec<_>>(),
            hash: HashMap::new()
        })
    }
}

impl Debug for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Cavern\n")?;
        write!(f,"Width:{}, Length:{}", self.width, self.lines)?;
        for (i,c) in self.layout.iter().enumerate() {
            if i % self.width == 0 {
                writeln!(f)?
            };
            write!(f," {}", *c as char)?;
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

        println!("{:?}",cavern.move_beam(0,D::East));
        println!("{:?}",cavern);
    }

    #[test]
    fn test_parse_cavern() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let cavern = inp.parse::<Cavern>().unwrap_or_default();
        println!("{:?}",cavern);
    }
}