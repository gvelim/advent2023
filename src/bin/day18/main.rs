mod instruction;
mod digplan;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use instruction::{Instruction, RGB, Direction};
use digplan::DigPlan;

fn main() {

}

type Depth = u8;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Position(isize,isize);

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
    map: BTreeMap<Position, Trench>
}

impl Default for Lagoon {
    fn default() -> Self {
        Lagoon { map: BTreeMap::new() }
    }
}

impl Lagoon {
    fn get_digger(&mut self, pos: Position, depth: Depth) -> Digger {
        Digger { lagoon:self, pos, depth }
    }
    fn min_pos(&self) -> Option<&Position> {
        self.map
            .first_key_value()
            .map(|(pos,_)| pos)
    }
    fn max_pos(&self) -> Option<&Position> {
        self.map
            .last_key_value()
            .map(|(pos,_)| pos)
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let min = self.min_pos().unwrap();
        let max = self.max_pos().unwrap();

        writeln!(f,"Lagoon")?;
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                write!(f,"{:2}",
                       if self.map.get(&Position(x,y)).is_some() {'#'} else {'.'}
                )?;
            };
            writeln!(f)?;
        };

        Ok(())
    }
}
struct Digger<'a> {
    lagoon: &'a mut Lagoon,
    pos: Position,
    depth: Depth
}

impl Digger<'_> {
    fn dig(&mut self, instr: &Instruction) -> usize {
        let Digger{ lagoon, pos, depth} = self;
        (0..instr.run)
            .take_while(|_|
                lagoon.map.insert(*pos.next(instr.dir), Trench(*depth, instr.rgb)).is_none()
            )
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dig() {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("ops");
        let plan = inp.parse::<DigPlan>().expect("failed to load Dig Plan");

        let mut lagoon = Lagoon::default();
        let mut digger = lagoon.get_digger(Position(0, 0), 1);

        let total = plan.iter()
            .map(|i| {
                assert_eq!(digger.dig(i), i.run);
                i.run
            })
            .sum::<usize>();

        println!("Steps: {total}\n{:?}",lagoon);
        println!("{:?}", lagoon.min_pos());
        println!("{:?}", lagoon.max_pos());
    }

}

