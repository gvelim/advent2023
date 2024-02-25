mod instruction;
mod digging_plan;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use instruction::{Instruction, RGB, Direction};
use digging_plan::DigPlan;

fn main() {

}

type Depth = u8;

type Unit = i16;

#[derive(Eq, PartialEq, Clone, Copy)]
struct Position(Unit,Unit);
impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{})",self.0,self.1)
    }
}
impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl Position {
    fn next(&mut self, dir: Direction) -> &mut Self {
        match dir {
            Direction::U => self.1 -= 1,
            Direction::R => self.0 += 1,
            Direction::D => self.1 += 1,
            Direction::L => self.0 -= 1,
        };
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct Trench(Depth, RGB);

struct Lagoon {
    min: Position,
    max: Position,
    map: BTreeMap<Position, Trench>
}

impl Default for Lagoon {
    fn default() -> Self {
        Lagoon {
            min: Position(Unit::MAX,Unit::MAX),
            max: Position(Unit::MIN,Unit::MIN),
            map: BTreeMap::new()
        }
    }
}

impl Lagoon {
    fn dig_trench(&mut self, pos: Position, trench: Trench) -> Option<Trench> {
        self.min.0 = std::cmp::min(self.min.0, pos.0);
        self.min.1 = std::cmp::min(self.min.1, pos.1);
        self.max.0 = std::cmp::max(self.max.0, pos.0);
        self.max.1 = std::cmp::max(self.max.1, pos.1);
        self.map.insert(pos, trench)
    }
    fn min_pos(&self) -> Option<Position> {
        Some(self.min)
    }
    fn max_pos(&self) -> Option<Position> {
        Some(self.max)
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"Lagoon")?;
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                write!(f,"{:2}",
                       if self.map.get(&Position(x,y)).is_some() {'#'} else {'.'}
                )?;
            };
            writeln!(f)?;
        };
        Ok(())
    }
}
struct Digger {
    pos: Position,
    depth: Depth
}

impl Digger {
    fn new(pos: Position, depth: Depth) -> Digger {
        Digger { pos, depth }
    }

    fn dig(&mut self, lagoon: &mut Lagoon, instr: &Instruction) -> usize {
        let Digger{ pos, depth} = self;
        (0..instr.run)
            .take_while(|_|
                lagoon.dig_trench(*pos.next(instr.dir), Trench(*depth, instr.rgb)).is_none()
            )
            .count()
    }
}

#[cfg(test)]
mod test {
    use std::ops::Bound::Included;
    use super::*;

    #[test]
    fn test_dig() {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("ops");
        let plan = inp.parse::<DigPlan>().expect("failed to load Dig Plan");

        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0), 1);

        let total = plan.iter()
            .map(|ins| {
                digger.dig(&mut lagoon, ins)
            })
            .sum::<usize>();

        println!("Steps: {total}\n{:?}",lagoon);
        println!("{:?}", lagoon.min_pos());
        println!("{:?}", lagoon.max_pos());

        lagoon.map
            .range(Position(Unit::MIN,0)..=Position(Unit::MAX,0))
            .for_each(|d| println!("{:?}",d))
    }

}

