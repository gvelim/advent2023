use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::repeat;
use std::str::FromStr;

type Position = usize;
type Energy = usize;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day16/input.txt").expect("Ops!");
    let mut cavern = inp.parse::<Cavern>().unwrap();

    let t = std::time::Instant::now();
    cavern.move_beam(0, D::Right);
    println!("Part 1 : Cavern Energy = {:?} - {:?}", cavern.measure_energy(), t.elapsed());
    assert_eq!(cavern.measure_energy(), 6902);

    let t = std::time::Instant::now();
    let m = entry_points(cavern.width,cavern.lines)
            .map(|(idx,dir)| {
                cavern.energise(idx,dir);
                cavern.measure_energy()
            })
            .max();
    println!("Part 2 : Max Energy = {:?} - {:?}", m, t.elapsed());
    assert_eq!(m,Some(7697));
}

fn entry_points(w:usize, h:usize) -> impl Iterator<Item=(Position, Direction)> + 'static {
    (0..w).zip(repeat(D::Down))
        .chain((w *(h-1)..w * h).zip(repeat(D::Up)))
        .chain((0..w * h).step_by(w).zip(repeat(D::Right)))
        .chain((0..w * h + 1).step_by(w).skip(1).map(|c| c-1).zip(repeat(D::Left)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum Direction { Up, Left, Down, Right, UpDown, LeftRight }
use Direction as D;

impl Direction {
    fn direction(&self, tile: u8) -> Direction {
        match (tile, self) {
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
            (_,d) => *d
        }
    }
}

pub(crate) struct Cavern {
    width: usize,
    lines: usize,
    con: std::rc::Rc<[u8]>,
    nrg: Vec<u8>,
    tail: HashMap<usize,Direction>
}

impl Cavern {
    fn measure_energy(&self) -> Energy {
        self.nrg.iter().filter(|c| b'#'.eq(c)).count()
    }
    fn energise(&mut self, idx: Position, dir:Direction) -> Option<Energy> {
        self.tail.clear();
        self.nrg.fill(b'.');
        self.move_beam(idx,dir)
    }
    fn step(&self, idx: Position, dir:Direction) -> Option<Position> {
        match dir {
            D::Right if idx % self.width < self.width-1 => Some(idx + 1),
            D::Left if idx % self.width > 0 => Some(idx - 1),
            D::Up if (self.width..self.con.len()).contains(&idx) => Some(idx - self.width),
            D::Down if idx < self.con.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
    fn move_beam(&mut self, idx: Position, dir:Direction) -> Option<Energy> {

        // Has the bean hit a contraption
        if self.con[idx] != b'.' {
            // Cycle Detection: have we enter the contraption from the same direction ?
            if Some(true) == self.tail.get(&idx).map(|d| dir.eq(d)) {
                return None
            }
            else {
                // Store beam direction at contraption point so we can detect the cycle
                self.tail.insert(idx,dir);
            }
        }

        // Energise cell
        self.nrg[idx] = b'#';

        // Find new direction based on current tile
        match dir.direction(self.con[idx]) {
            d@(D::Right | D::Left | D::Up | D::Down) =>
                self.step(idx, d).and_then(|next| self.move_beam(next, d)),
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
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cavern {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            con: s.lines()
                .flat_map(|line| line.bytes())
                .collect::<std::rc::Rc<[u8]>>(),
            nrg: vec![b'.'; s.len()],
            tail: HashMap::new()
        })
    }
}

impl Debug for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Cavern")?;
        writeln!(f,"Width:{}, Length:{}", self.width, self.lines)?;
        writeln!(f,"{a:w$}  {b:w$}", w=self.width<<1, a = "Contraption", b = "Energy")?;
        writeln!(f,"{a:-<w$}  {a:-<w$}", w=self.width<<1, a="")?;

        let mut citer = self.con.iter();
        let mut eiter = self.nrg.iter();
        for _ in 0..self.lines {
            for _ in 0..self.width { write!(f, "{:2}", *citer.next().unwrap() as char)? };
            write!(f, "  ")?;
            for _ in 0..self.width { write!(f, "{:2}", *eiter.next().unwrap() as char)? };
            writeln!(f)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max_energy() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let mut cavern = inp.parse::<Cavern>().unwrap();

        let m = entry_points(cavern.width,cavern.lines)
            .inspect(|d| print!("{:?} -> ",d))
            .map(|(idx,dir)| {
                cavern.energise(idx,dir);
                // println!("{:?}",cavern);
                cavern.measure_energy()
            })
            .inspect(|d| println!("{d:2}"))
            .max();
        assert_eq!(m,Some(51));
    }
    #[test]
    fn test_move_bean() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let mut cavern = inp.parse::<Cavern>().unwrap();

        println!("{:?}",cavern.move_beam(0,D::Right));
        println!("{:?}",cavern);
        assert_eq!(cavern.measure_energy(), 46);
    }

    #[test]
    fn test_parse_cavern() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let cavern = inp.parse::<Cavern>().unwrap();
        println!("{:?}",cavern);
    }
}