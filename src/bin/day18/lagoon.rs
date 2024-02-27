use crate::instruction::{Instruction, Rgb};
use crate::position::{Position, Unit};
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

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
                    .dig_trench(*self.pos.next(instr.dir), Trench(self.depth, instr.rgb))
                    .is_none()
            })
            .count()
    }
}

#[derive(Debug, Copy, Clone)]
struct Trench(Depth, Rgb);

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

    fn calculate_area(&self) -> usize {
        let mut last: Option<(Position, Trench)> = None;

        (self.min.1..=self.max.1).map(|y| {
            println!("Line {y}");
            self.map
                .range(Position(Unit::MIN, y)..=Position(Unit::MAX, y))
                .filter_map(|(p, t)| {
                    if let Some((lp, lt)) = last {
                        last = Some((*p,*t));
                        if p.0 - lp.0 > 1 {
                            Some((lp, p))
                        } else {
                            None
                        }
                    } else {
                        last = Some((*p,*t));
                        None
                    }
                })
                .inspect(|(p,t)| print!("Out\t{:?} -> {:?} = ", p, t))
                .map(|(a,b)| (b.0 - a.0 - 1) as usize)
                .inspect(|n| println!("{n}"))
                .sum::<usize>()
        })
        .sum::<usize>()
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lagoon")?;
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                write!(f, "{:2}",
                    if self.map.get(&Position(x, y)).is_some() {'#'} else {'.'}
                )?;
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
