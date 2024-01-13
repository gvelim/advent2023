use std::fmt::{Debug, Formatter, Write};
use std::rc::Rc;
use std::str::FromStr;

fn main() {

}

#[derive(Copy,Clone)]
enum Direction { East, West, North, South }

#[derive(Default)]
struct ReflectorDish {
    width: usize,
    lines: usize,
    layout: Vec<char>
}

impl ReflectorDish {
    fn to_idx(&self, pos: (usize,usize)) -> Option<usize> {
        if pos.1 < self.lines && pos.0 < self.width { Some(pos.1 * self.lines + pos.0) } else { None }
    }
    fn to_cartesian(&self, idx:usize) -> Option<(usize,usize)> {
        if idx < self.lines * self.width { Some((idx % self.width, idx / self.lines)) } else { None }
    }
    fn move_rock(&mut self, idx: usize, dir:Direction) -> Option<usize> {
        if idx >= self.layout.len() { return None }
        match dir {
            Direction::East if idx < self.layout.len() => Some(idx+1),
            Direction::West if idx > 0 => Some(idx - 1),
            Direction::North if idx > self.width => Some(idx - self.width),
            Direction::South if idx < self.layout.len() - self.width => Some(idx + self.width),
            _ => None
        }
            .and_then(|next|{
                if self.layout[next] == '.' {
                    self.layout.swap(idx,next);
                    self.move_rock(next,dir)
                } else {
                    return Some(idx / self.lines)
                }
            })
            .or( Some(idx / self.lines) )
    }
    fn tilt(&mut self, dir: Direction) -> usize {
        let rocks = self.round_rocks().collect::<Rc<[usize]>>();

        rocks.into_iter()
            .inspect(|s| print!("idx: {s} -> "))
            .map(|&r| {
                self.move_rock(r,dir).map(|cost| self.lines - cost).unwrap()
            })
            .inspect(|s| println!("{s}"))
            .sum::<usize>()
    }
    fn round_rocks(&self) -> impl Iterator<Item=usize> + '_ {
        self.layout.iter()
            .enumerate()
            .filter(|&(_,c)| *c == 'O')
            .map(|(idx,_)| idx )
    }
}

impl FromStr for ReflectorDish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReflectorDish {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            layout: s.lines()
                .flat_map(|line| line.chars())
                .collect::<Vec<char>>()
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
            f.write_char(*c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tilt() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = &mut inp.parse::<ReflectorDish>().unwrap_or_default();

        println!("{:?}",dish);
        assert_eq!(dish.tilt(Direction::North),136);
        println!("{:?}",dish);
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
        println!("{:?}",dish.round_rocks().collect::<Rc<[_]>>());
        assert_eq!(
            dish.round_rocks().collect::<Rc<[_]>>(),
            [0, 1, 2, 3, 10, 11, 21, 30, 34, 39, 47, 50, 55, 62, 66, 69, 72, 77].into()
        )
    }
    #[test]
    fn test_extract_rocks() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();

        println!("{:?}",dish.round_rocks().collect::<Rc<[_]>>());

    }
    #[test]
    fn test_cartesian_to_index() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();

        assert_eq!(Some((0,0)), dish.to_cartesian(0));
        assert_eq!(Some((0,1)), dish.to_cartesian(10));
        assert_eq!(Some((1,1)), dish.to_cartesian(11));
        assert_eq!(None, dish.to_cartesian(1000));
        assert_eq!(dish.to_idx((0,0)), Some(0));
        assert_eq!(dish.to_idx((0,1)), Some(10));
        assert_eq!(dish.to_idx((1,1)), Some(11));
        assert_eq!(dish.to_idx((20,1)), None);
        assert_eq!(dish.to_idx((0,20)), None);
    }
    #[test]
    fn test_parse_reflector_dish() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");
        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();
        println!("{:?}",dish);
    }

}