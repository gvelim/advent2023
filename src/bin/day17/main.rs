use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::str::FromStr;

fn main() {}

#[derive(Debug, Clone, Copy)]
enum Direction { Up, Right, Down, Left }
impl Direction {
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right_mut(&mut self) -> Direction {
        *self = self.turn_right();
        *self
    }
    fn turn_left_mut(&mut self) -> Direction {
        *self = self.turn_left();
        *self
    }
}

type Position = usize;

struct Crucible<'a> {
    map: &'a CityMap,
    pos: Position,
    dir: Direction,
    straight: u8
}
impl Crucible<'_> {
    fn neighbour_blocks(&mut self) -> impl Iterator<Item=(Direction,Option<Position>)> + '_ {
        match self.dir {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Right => [Direction::Right, Direction::Up, Direction::Down],
            Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Left, Direction::Up, Direction::Down],
        }
            .into_iter()
            .map(|dir| (dir, self.map.step_onto(self.pos,dir)))
    }
}
impl Iterator for Crucible<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        // get heat from neighbour city-blocks
        // pick block where heat_loss = min(block1, block2, block3)
        // move onto the new block
        self.map.step_onto(self.pos,self.dir)
            .map(|d| {self.pos = d; d})
    }
}

type Heat = u8;

struct CityMap {
    width: usize,
    lines: usize,
    map: Rc<[Heat]>,
}
impl CityMap {
    fn get_crucible(&self, pos: Position, dir: Direction) -> Crucible {
        Crucible { map: self, pos, dir, straight:0 }
    }
    fn step_onto(&self, from: Position, dir: Direction) -> Option<Position> {
        use Direction as D;
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
    fn test_calc_min_path() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        println!("{:?}",map);

    }
    #[test]
    fn test_neighbour_blocks() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let mut cr = map.get_crucible(13,Direction::Right);

        cr.neighbour_blocks().for_each(|d| println!("{:?}",d));
        cr.next();
        cr.neighbour_blocks().for_each(|d| println!("{:?}",d));

    }
    #[test]
    fn test_step_onto() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        assert_eq!(map.step_onto(0,Direction::Left), None);
        assert_eq!(map.step_onto(0,Direction::Up), None);
        assert_eq!(map.step_onto(0,Direction::Down), Some(13));
        assert_eq!(map.step_onto(0,Direction::Right), Some(1));
        assert_eq!(map.step_onto(1,Direction::Down), Some(14));
        assert_eq!(map.step_onto(14,Direction::Left), Some(13));
        assert_eq!(map.step_onto(13,Direction::Left), None);
        assert_eq!(map.step_onto(168,Direction::Left), Some(167));
        assert_eq!(map.step_onto(168,Direction::Up), Some(155));
        assert_eq!(map.step_onto(168,Direction::Right), None);
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
