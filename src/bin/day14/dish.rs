use std::fmt::{Debug, Formatter, Write};
use std::rc::Rc;
use std::str::FromStr;
use Direction as D;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Direction { North, West, South, East }

#[derive(Default)]
pub(crate) struct ReflectorDish {
    width: usize,
    lines: usize,
    pub(crate) layout: Vec<u8>
}

type Position = usize;
type Cost = usize;

impl ReflectorDish {
    fn next(&self, idx: usize, dir:Direction) -> Option<Position> {
        match dir {
            D::East if idx % self.width < self.width - 1 => Some(idx + 1),
            D::West if idx % self.width > 0 => Some(idx - 1),
            D::North if idx > self.width - 1 => Some(idx - self.width),
            D::South if idx < self.layout.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
    fn move_rock(&mut self, idx: usize, dir:Direction) -> Option<Cost> {
        if idx >= self.layout.len() { return None }
        if let Some(next) = self.next(idx,dir) {
            if self.layout[next] == b'.' {
                self.layout.swap(idx, next);
                return self.move_rock(next, dir)
            }
        }
        Some(idx / self.lines)
    }
    pub(crate) fn tilt(&mut self, dir: Direction) -> Cost {
        match dir {
            D::East => self.round_rocks_w2e().rev().collect::<Rc<[Position]>>(),
            D::West => self.round_rocks_w2e().collect::<Rc<[Position]>>(),
            D::North => self.round_rocks_n2s().collect::<Rc<[Position]>>(),
            D::South => self.round_rocks_n2s().rev().collect::<Rc<[Position]>>(),
        }
            .iter()
            // .inspect(|s| print!("idx: {s} -> "))
            .map(|&index| self
                .move_rock(index, dir)
                .map(|cost| self.lines - cost)
                .unwrap()
            )
            // .inspect(|s| println!("{s}"))
            .sum::<Cost>()
    }
    pub(crate) fn spin_cycle(&mut self) -> Cost {
        [D::North,D::West,D::South,D::East]
            .into_iter()
            .map(|dir| self.tilt(dir))
            .last()
            .unwrap()
    }
    pub(crate) fn spin_cycle_nth(&mut self, nth: usize) -> Option<Cost> {
        let mut map = std::collections::HashMap::<Vec<u8>,usize>::new();

        (1..nth)
            .map(|cycle| (
                cycle,
                self.spin_cycle(),
                map.insert(self.layout.clone(),cycle)
            ))
            .skip_while(|(cycle, _, seen)|
                seen.map(|last| {
                    (nth - last) % (cycle - last) != 0
                }).unwrap_or(true)
            )
            .map(|(_,cost,_)| cost)
            .next()
    }
    fn round_rocks_n2s(&self) -> impl DoubleEndedIterator<Item=Position> + '_ {
        self.layout.iter()
            .enumerate()
            .filter(|&(_,c)| *c == b'O')
            .map(|(idx,_)| idx )
    }
    fn round_rocks_w2e(&self) -> impl DoubleEndedIterator<Item=Position> + '_ {
        (0..self.width)
            .flat_map(move |x|{
                (0..self.lines).map(move |y| y * self.lines + x )
            })
            .filter(|&idx| self.layout[idx] == b'O')
    }
}

impl FromStr for ReflectorDish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReflectorDish {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            layout: s.lines()
                .flat_map(|line| line.bytes())
                .collect::<Vec<_>>()
        })
    }
}

impl Debug for ReflectorDish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReflectorDish\n")?;
        f.write_str(&format!("Width:{}, Length:{}", self.width, self.lines))?;
        for (i,c) in self.layout.iter().enumerate() {
            if i % self.width == 0 {
                f.write_char('\n')?
            };
            f.write_char(' ')?;
            f.write_char(*c as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tilt_cycle() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

        let cost = dish.spin_cycle_nth(1000000000);
        println!("Cost after 1000000000 cycles: {:?}",cost);
        assert_eq!(Some(64),cost);
    }
    #[test]
    fn test_tilt() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

        let data = [(Direction::North, 136), (Direction::West, 136), (Direction::South, 87), (Direction::East, 87)];

        println!("{:?}",dish);
        for (dir,out) in data.into_iter() {
            let ret = dish.tilt(dir);
            println!("{:?} -> {ret} = {:?}",dir, dish);
            assert_eq!(ret,out);
        }
    }
    #[test]
    fn test_move_rock() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

        println!("{:?}",dish);
        assert_eq!(dish.move_rock(10,D::North), Some(1));
        assert_eq!(dish.move_rock(12,D::North), Some(0));
        assert_eq!(dish.move_rock(13,D::North), Some(0));
        assert_eq!(dish.move_rock(31,D::North), Some(0));
        assert_eq!(dish.move_rock(41,D::North), Some(1));
        assert_eq!(dish.move_rock(91,D::North), Some(2));
        assert_eq!(dish.move_rock(92,D::North), Some(7));
        assert_eq!(dish.move_rock(120,D::North), None);
        println!("{:?}",dish);
        println!("{:?}",dish.round_rocks_n2s().collect::<Rc<[_]>>());
        assert_eq!(
            dish.round_rocks_n2s().collect::<Rc<[_]>>(),
            [0, 1, 2, 3, 10, 11, 21, 30, 34, 39, 47, 50, 55, 62, 66, 69, 72, 77].into()
        )
    }
    #[test]
    fn test_extract_rocks_north_to_south() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();

        assert_eq!(
            dish.round_rocks_n2s().collect::<Rc<[_]>>(),
            [0, 10, 12, 13, 30, 31, 34, 39, 41, 47, 50, 55, 62, 66, 69, 77, 91, 92].into()
        );

    }
    #[test]
    fn test_extract_rocks_east_to_west() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();

        assert_eq!(
            dish.round_rocks_w2e().collect::<Rc<[_]>>(),
            [0, 10, 30, 50, 31, 41, 91, 12, 62, 92, 13, 34, 55, 66, 47, 77, 39, 69].into()
        );

    }
    #[test]
    fn test_parse_reflector_dish() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();
        println!("{:?}",dish);
    }
}