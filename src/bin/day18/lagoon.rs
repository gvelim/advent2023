use crate::instruction::{Direction, Instruction, Rgb};
use crate::position::{Position, Unit};
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;

type Depth = u8;

pub(crate) struct Digger {
    pos: Position,
    depth: Depth,
}

impl Digger {
    pub fn new(pos: Position, depth: Depth) -> Digger {
        Digger { pos, depth }
    }

    pub fn dig(&mut self, lagoon: &mut Lagoon, instr: &Instruction) -> usize {
        (0..instr.run)
            .take_while(|_| {
                lagoon
                    .dig_trench(*self.pos.next_mut(instr.dir), Trench(self.depth, instr.rgb))
                    .is_none()
            })
            .count()
    }
}

#[derive(Debug, Copy, Clone)]
struct Trench(Depth, Rgb);

impl Trench {
    fn r(&self) -> u8 {
        self.1 .0
    }
    fn g(&self) -> u8 {
        self.1 .1
    }
    fn b(&self) -> u8 {
        self.1 .2
    }
    fn rgb(&self) -> usize {
        (self.r() << 16 + self.b() << 8 + self.g()) as usize
    }
    fn depth(&self) -> Depth {
        self.0
    }
}

pub(crate) struct Lagoon {
    min: Position,
    max: Position,
    map: BTreeMap<Position, Trench>,
}

impl Default for Lagoon {
    fn default() -> Self {
        Lagoon {
            min: Position(Unit::MAX, Unit::MAX),
            max: Position(Unit::MIN, Unit::MIN),
            map: BTreeMap::new(),
        }
    }
}

impl Lagoon {
    fn min_pos(&self) -> Position {
        self.min
    }

    fn max_pos(&self) -> Position {
        self.max
    }

    fn dig_trench(&mut self, pos: Position, trench: Trench) -> Option<Trench> {
        self.min.0 = std::cmp::min(self.min.0, pos.0);
        self.min.1 = std::cmp::min(self.min.1, pos.1);
        self.max.0 = std::cmp::max(self.max.0, pos.0);
        self.max.1 = std::cmp::max(self.max.1, pos.1);
        self.map.insert(pos, trench)
    }

    fn get_line_intersections(&self, line: Unit) -> impl Iterator<Item = RangeInclusive<Unit>> + '_ {
        let mut last: Option<Position> = None;

        self.map
            .range(Position(Unit::MIN, line)..=Position(Unit::MAX, line))
            .filter_map(move |(p, _)| {
                let next = self.map.get(&p.next(Direction::R));
                let out = match (last, next) {
                    (None,None) => Some(p.0..=p.0),
                    (None, Some(_)) => {
                        last = Some(*p);
                        None
                    },
                    (Some(lp), None) => {
                        last = None;
                        Some(lp.0..=p.0)
                    },
                    _ => None,
                };
                out
            })
    }

    fn calculate_area(&self) -> usize {
        (self.min.1..=self.max.1)
            .map(|y| {
                print!("Line {y}\n\t");
                self.get_line_intersections(y)
                    // .array_chunks::<2>()
                    .inspect(|p| print!("{:?}, ", p))
                    // .map(|pair| (pair[1].start() - pair[0].end()) as usize)
                    .map(|p| *p.start() as usize)
                    .sum::<usize>()
            })
            .inspect(|s| println!(" = {s}"))
            .sum::<usize>()
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use colored::*;

        writeln!(f, "Lagoon")?;
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                write!(
                    f,
                    "{:2}",
                    &self
                        .map
                        .get(&Position(x, y))
                        .map(|t| "#".truecolor(t.r(), t.g(), t.b()))
                        .unwrap_or(".".into())
                )?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::digging_plan::DigPlan;

    #[test]
    fn test_lagoon_area() {
        let plan = std::fs::read_to_string("./src/bin/day18/sample.txt")
            .expect("ops")
            .parse::<DigPlan>()
            .expect("failed to load Dig Plan");

        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0), 1);

        let total = plan
            .iter()
            .map(|ins| digger.dig(&mut lagoon, ins))
            .sum::<usize>();

        println!("{:?}\nTrench {total}", lagoon);
        println!("Lagoon area {}", lagoon.calculate_area() + total);
    }

    #[test]
    fn test_dig() {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("ops");
        let plan = inp.parse::<DigPlan>().expect("failed to load Dig Plan");

        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0), 1);

        let total = plan
            .iter()
            .map(|ins| digger.dig(&mut lagoon, ins))
            .sum::<usize>();

        println!("Steps: {total}\n{:?}", lagoon);
        println!("{:?}", lagoon.min_pos());
        println!("{:?}", lagoon.max_pos());
    }
}
