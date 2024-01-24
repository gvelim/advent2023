use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter::repeat;
use std::str::FromStr;
use crate::direction::Direction;

type Position = usize;
type Energy = usize;

pub(crate) fn entry_points(w:usize, h:usize) -> impl Iterator<Item=(Position, Direction)> + 'static {
    use Direction as D;

    (0..w).zip(repeat(D::Down))
        .chain((w *(h-1)..w * h).zip(repeat(D::Up)))
        .chain((0..w * h).step_by(w).zip(repeat(D::Right)))
        .chain((0..w * h + 1).step_by(w).skip(1).map(|c| c-1).zip(repeat(D::Left)))
}

pub(crate) struct Cavern {
    pub(crate) width: usize,
    pub(crate) lines: usize,
    con: std::rc::Rc<[u8]>,
    nrg: Vec<u8>,
    tail: HashMap<usize,Vec<Direction>>
}

impl Cavern {
    pub(crate) fn measure_energy(&self) -> Energy {
        self.nrg.iter().filter(|c| b'#'.eq(c)).count()
    }
    pub(crate) fn energise(&mut self, idx: Position, dir:Direction) {
        self.tail.clear();
        self.nrg.fill(b'.');
        self.move_beam(idx,dir)
    }
    fn step(&self, idx: Position, dir:Direction) -> Option<Position> {
        use Direction as D;
        match dir {
            D::Right if idx % self.width < self.width-1 => Some(idx + 1),
            D::Left if idx % self.width > 0 => Some(idx - 1),
            D::Up if (self.width..self.con.len()).contains(&idx) => Some(idx - self.width),
            D::Down if idx < self.con.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }

    fn move_beam(&mut self, idx: Position, dir:Direction) {
        use Direction as D;

        let tile = self.con[idx];

        // Has the light-beam fallen into a circle ?
        if self.has_entered_cycle(tile, idx, dir) { return }

        // Energise cell
        self.nrg[idx] = b'#';

        // Find new direction based on current tile
        match dir.next(tile) {
            D::LeftRight => {
                let _ = self.step(idx, D::Left).is_some_and(|pos| self.move_beam(pos, D::Left) == ());
                let _ = self.step(idx, D::Right).is_some_and(|pos| self.move_beam(pos, D::Right) == ());
            },
            D::UpDown => {
                let _ = self.step(idx, D::Down).is_some_and(|pos| self.move_beam(pos, D::Down) == ());
                let _ = self.step(idx, D::Up).is_some_and(|pos| self.move_beam(pos, D::Up) == ());
            },
            d => {
                let _ = self.step(idx, d).is_some_and(|pos| self.move_beam(pos, d) == ());
            }
        }
    }

    fn has_entered_cycle(&mut self, tile: u8, idx: Position, dir: Direction) -> bool {
        use Direction as D;

        if tile == b'.' { return false }

        // Cycle Detection: have we enter the contraption from the same direction ?
        if Some(true) == self.tail.get(&idx).map(|d| d.contains(&dir)) { return true }

        // Store beam direction at contraption point so we can detect the cycle
        // Optimise around splitters by storing both opposite directions
        // hence we stop re-entering the cycle from the opposite direction
        let store = match (tile,dir) {
            (b'-'|b'|', D::Up| D::Down) => [D::Up, D::Down],
            (b'-'|b'|', D::Left| D::Right) => [D::Left, D::Right],
            _ => [dir,dir]
        };

        self.tail.entry(idx)
            .and_modify(|v| v.extend(store))
            .or_insert(Vec::default())
            .extend(store);

        return false;
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
    use Direction as D;

    #[test]
    fn test_find_max_energy() {
        let inp = std::fs::read_to_string("src/bin/day16/sample.txt").expect("Ops!");
        let mut cavern = inp.parse::<Cavern>().unwrap();

        let m = entry_points(cavern.width,cavern.lines)
            .inspect(|d| print!("{:?} -> ",d))
            .map(|(idx,dir)| {
                cavern.energise(idx,dir);
                println!("{:?}",cavern);
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

        println!("{:?}",cavern.energise(0,D::Right));
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