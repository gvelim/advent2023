use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day14/input.txt").expect("Ops!");
    let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

    let t = std::time::Instant::now();
    println!("Part 1: Total load = {:?} - {:?}",dish.tilt(Direction::North),t.elapsed());

    let mut map = HashMap::<Vec<u8>,usize>::new();

    let t = std::time::Instant::now();
    let cost = (1..1000)
        .map(|round| (
            round,
            dish.spin_cycle(),
            map.insert(dish.layout.clone(), round)
        ))
        .skip_while(|(round, _, seen)|
            seen.map(|last|
                (1000000000 - last) % (round - last) != 0
            ).unwrap_or(true)
        )
        .map(|(_,cost,_)| cost)
        .next();

    println!("Part 2: Total load = {:?} - {:?}", cost, t.elapsed()
    );
}

#[derive(Copy, Clone, Debug)]
enum Direction { North, West, South, East }

#[derive(Default)]
struct ReflectorDish {
    width: usize,
    lines: usize,
    layout: Vec<u8>
}

impl ReflectorDish {
    fn next(&self, idx: usize, dir:Direction) -> Option<usize> {
        match dir {
            Direction::East if idx < (idx/self.lines)*self.width + self.width - 1 => Some(idx+1),
            Direction::West if idx > (idx/self.lines)*self.width => Some(idx - 1),
            Direction::North if (self.width..self.layout.len()).contains(&idx) => Some(idx - self.width),
            Direction::South if idx < self.layout.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
    fn move_rock(&mut self, idx: usize, dir:Direction) -> Option<usize> {
        if idx >= self.layout.len() { return None }
        self.next(idx,dir)
            .and_then(|next|{
                if self.layout[next] == b'.' {
                    self.layout.swap(idx,next);
                    self.move_rock(next,dir)
                } else {
                    return Some(idx / self.lines)
                }
            })
            .or( Some(idx / self.lines) )
    }
    fn tilt(&mut self, dir: Direction) -> usize {
        let rocks = match dir {
            Direction::East => self.round_rocks_w2e().rev().collect::<Rc<[usize]>>(),
            Direction::West => self.round_rocks_w2e().collect::<Rc<[usize]>>(),
            Direction::North => self.round_rocks_n2s().collect::<Rc<[usize]>>(),
            Direction::South => self.round_rocks_n2s().rev().collect::<Rc<[usize]>>(),
        };

        rocks.into_iter()
            // .inspect(|s| print!("idx: {s} -> "))
            .map(|&r| self
                .move_rock(r,dir)
                .map(|cost| self.lines - cost)
                .unwrap()
            )
            // .inspect(|s| println!("{s}"))
            .sum::<usize>()
    }
    fn spin_cycle(&mut self) -> usize {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East)
    }
    fn round_rocks_n2s(&self) -> impl DoubleEndedIterator<Item=usize> + '_ {
        self.layout.iter()
            .enumerate()
            .filter(|&(_,c)| *c == b'O')
            .map(|(idx,_)| idx )
    }
    fn round_rocks_w2e(&self) -> impl DoubleEndedIterator<Item=usize> + '_ {
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

        let mut map = HashMap::<Vec<u8>,usize>::new();

        let cost = (1..1000)
            .map(|idx| (
                idx,
                dish.spin_cycle(),
                map.insert(dish.layout.clone(),idx)
            ))
            .inspect(|d| println!("{:?}",d))
            .skip_while(|(idx, _, seen)|
                seen.map(|last|
                    (1000000000 - last) % (idx - last) != 0
                ).unwrap_or(true)
            )
            .next();
        println!("Cost after 1000000000 cycles: {:?}",cost);
        assert_eq!(Some(64),cost.map(|(_,cost,_)| cost));
    }
    #[test]
    fn test_tilt() {
        use crate::Direction::{East, North, South, West};

        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

        let data = [(North,136), (West,136), (South,87), (East,87)];

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
        assert_eq!(dish.move_rock(10,Direction::North), Some(1));
        assert_eq!(dish.move_rock(12,Direction::North), Some(0));
        assert_eq!(dish.move_rock(13,Direction::North), Some(0));
        assert_eq!(dish.move_rock(31,Direction::North), Some(0));
        assert_eq!(dish.move_rock(41,Direction::North), Some(1));
        assert_eq!(dish.move_rock(91,Direction::North), Some(2));
        assert_eq!(dish.move_rock(92,Direction::North), Some(7));
        assert_eq!(dish.move_rock(120,Direction::North), None);
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