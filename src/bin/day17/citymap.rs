use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::str::FromStr;
use crate::{crucible::{Crucible, Position}, direction::Direction};
use Direction as D;

type Heat = u8;

pub(crate) struct CityMap {
    width: usize,
    lines: usize,
    pub(crate) map: Rc<[Heat]>,
}
impl CityMap {
    pub(crate) fn get_crucible(&self, pos: Position, dir: D) -> Crucible {
        Crucible::new(self,pos,dir)
    }
    pub(crate) fn step_onto(&self, from: Position, dir: D) -> Option<Position> {
        if from >= self.map.len() { return None }
        match dir {
            D::Right if from % self.width < self.width-1 => Some(from + 1),
            D::Left if from % self.width > 0 => Some(from - 1),
            D::Up if from > self.width - 1 => Some(from - self.width),
            D::Down if from < self.map.len() - self.width => Some(from + self.width),
            _ => None
        }
    }
}

impl FromStr for CityMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CityMap {
            width: s.lines().next().unwrap().len(),
            lines: s.lines().count(),
            map: s
                .lines()
                .flat_map(|line| line.bytes())
                .map(|c| c - b'0')
                .collect::<Rc<_>>(),
        })
    }
}
impl Debug for CityMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"CityMap")?;
        write!(f,"Width:{}, Lines:{}",self.width,self.lines)?;
        for idx in 0..self.map.len() {
            if idx % self.width == 0 { writeln!(f)?; }
            write!(f, "{:3}", self.map[idx])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_onto() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let data = [
            ((0, D::Left), None),
            ((0, D::Up), None),
            ((0, D::Down), Some(13)),
            ((0, D::Right), Some(1)),
            ((12, D::Left), Some(11)),
            ((12, D::Up), None),
            ((12, D::Down), Some(25)),
            ((12, D::Right), None),
            ((1, D::Down), Some(14)),
            ((14, D::Left), Some(13)),
            ((13, D::Left), None),
            ((168, D::Left), Some(167)),
            ((168, D::Up), Some(155)),
            ((168, D::Right), None),
            ((168, D::Down), None)
        ];
        for ((p,d),out) in data {
            assert_eq!(map.step_onto(p, d), out);
        }
    }
    #[test]
    fn test_parse_map() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        println!("{}",map.map.len());
        assert_eq!(
            map.map,
            [
                2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3,
                3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3,
                3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4,
                3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2,
                4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6,
                1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4,
                4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6,
                3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3,
                4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7,
                4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3,
                1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3,
                2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5,
                4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3
            ].into()
        )
    }
}