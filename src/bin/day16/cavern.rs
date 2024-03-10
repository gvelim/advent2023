use std::fmt::{Debug, Formatter};
use std::iter::repeat;
use std::str::FromStr;
use crate::direction::Direction;
use std::boxed::Box;

type Position = usize;
type Energy = usize;

pub(crate) fn entry_points(w:usize, h:usize) -> impl Iterator<Item=(Position, Direction)> + 'static {
    use Direction as D;
    let len = w*h;
    (0..w).zip(repeat(D::Down))
        .chain((len-w..len).zip(repeat(D::Up)))
        .chain((0..h).map(move |l| l*w).zip(repeat(D::Right)))
        .chain((1..=h).map(move |l| l*w-1).zip(repeat(D::Left)))
}

pub(crate) struct Cavern {
    pub(crate) width: usize,
    pub(crate) lines: usize,
    con: Box<[u8]>,
    nrg: Box<[(bool,Vec<Direction>)]>
}

impl Cavern {
    pub(crate) fn measure_energy(&self) -> Energy {
        self.nrg.iter().filter(|(c,_)| *c).count()
    }
    pub(crate) fn energise(&mut self, idx: Position, dir:Direction) {
        self.nrg.iter_mut().for_each(|(n,v)|{ *n = false; v.clear(); });
        self.move_beam(idx,dir)
    }
    fn step(&self, idx: Position, dir:Direction) -> Option<Position> {
        use Direction as D;
        match dir {
            D::Right if idx % self.width < self.width-1 => Some(idx + 1),
            D::Left if idx % self.width > 0 => Some(idx - 1),
            D::Up if idx > self.width - 1 => Some(idx - self.width),
            D::Down if idx < self.con.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }

    fn move_beam(&mut self, idx: Position, dir:Direction) {
        use Direction as D;

        // Has the light-beam fallen into a circle ?
        if self.con[idx] != b'.' && self.has_entered_cycle(idx, dir) { return }

        // Energise cell
        self.nrg[idx].0 = true;

        // Find new direction based on current tile
        match dir.next(self.con[idx] ) {
            D::LeftRight => {
                if let Some(pos) = self.step(idx, D::Left) { self.move_beam(pos, D::Left) };
                if let Some(pos) = self.step(idx, D::Right) { self.move_beam(pos, D::Right) };
            },
            D::UpDown => {
                if let Some(pos) = self.step(idx, D::Down) { self.move_beam(pos, D::Down) };
                if let Some(pos) = self.step(idx, D::Up) { self.move_beam(pos, D::Up) };
            },
            any =>
                if let Some(pos) = self.step(idx, any) { self.move_beam(pos, any) }
        }
    }

    fn has_entered_cycle(&mut self, idx: Position, dir: Direction) -> bool {
        use Direction as D;

        // Cycle Detection: have we enter the contraption from the same direction before ?
        if self.nrg[idx].1.contains(&dir) { return true }

        // Store light-beam direction at contraption point, for cycles detection
        // Optimise around splitters by storing both opposite directions
        // this stops us from re-entering the cycle from the opposite direction
        match (self.con[idx],dir) {
            (b'-', D::Up|D::Down) => self.nrg[idx].1.extend_from_slice(&[D::Up, D::Down,]),
            (b'|', D::Left|D::Right) => self.nrg[idx].1.extend_from_slice(&[D::Left, D::Right]),
            _ => self.nrg[idx].1.push(dir)
        };
        // println!("{:?} -> {:?}",(self.con[idx] as char,dir, idx), self.nrg[idx].1);
        false
    }
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cavern {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            con: s.lines().flat_map(|line| line.bytes()).collect::<Box<[_]>>(),
            nrg: s.lines().flat_map(|line| line.bytes()).map(|_| (false,Vec::with_capacity(4))).collect::<Box<[_]>>()
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
            for _ in 0..self.width { write!(f, "{:2}", if eiter.next().unwrap().0 {'#'} else {'.'} )? };
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
