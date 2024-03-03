use crate::instruction::{Direction, Instruction, Rgb};
use crate::position::{Position, Unit};
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::usize;

type Depth = u8;

#[derive(Debug, Copy, Clone)]
struct Trench(Depth, Rgb, Direction);

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
                    .dig_trench(
                        *self.pos.next_mut(instr.dir),
                        Trench(self.depth, instr.rgb, instr.dir),
                    )
                    .is_none()
            })
            .count()
    }
}

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
    fn dig_trench(&mut self, pos: Position, trench: Trench) -> Option<Trench> {
        self.min.0 = std::cmp::min(self.min.0, pos.0);
        self.min.1 = std::cmp::min(self.min.1, pos.1);
        self.max.0 = std::cmp::max(self.max.0, pos.0);
        self.max.1 = std::cmp::max(self.max.1, pos.1);
        self.map.insert(pos, trench)
    }

    pub(crate) fn calc_line_area(&self, line: Unit) -> usize {
        use Direction as D;
        let mut last: Option<(&Position, &Direction)> = None;

        self.map
            .range(Position(Unit::MIN, line)..=Position(Unit::MAX, line))
            .filter_map(|(p, Trench(depth, _, d))| -> Option<usize> {
                let mut out = None;
                if let Some((lp, ld)) = last {
                    out = match (ld, d) {
                        (D::U, D::D) | (D::U, D::L) | (D::R, D::D) | (D::R, D::L) => {
                            print!(", ({:?},{:?},{})", ld, d, p.0 - lp.0 - 1);
                            Some(*depth as usize * (p.0 - lp.0 - 1) as usize)
                        }
                        _ => None,
                    }
                }
                last = Some((p, d));
                out
            })
            .sum::<usize>()
    }

    pub(crate) fn calculate_area(&self) -> usize {
        (self.min.1..=self.max.1)
            .inspect(|y| print!("Line: {y}"))
            .map(|y| self.calc_line_area(y))
            .inspect(|p| println!(", Sum: {p}"))
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
    use std::rc::Rc;

    use super::*;
    use crate::digging_plan::DigPlan;

    #[test]
    fn test_lagoon_area() {
        let plan = match load_plan() {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let lagoon = dig_lagoon(&plan);
        let area = lagoon.calculate_area();

        println!(
            "{:?}\n\
            Trench     : {}\n\
            Lagoon area: {area}\n\
            Total      : {}",
            lagoon,
            lagoon.map.len(),
            lagoon.map.len() + area
        );
        assert_eq!(lagoon.map.len() + area, 62)
    }

    #[test]
    fn test_dig_lagoon() {
        let plan = match load_plan() {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let lagoon = dig_lagoon(&plan);

        println!("Steps: {}\n{:?}", lagoon.map.len(), lagoon);
        assert_eq!(lagoon.map.len(), 38)
    }

    fn dig_lagoon(plan: &DigPlan) -> Lagoon {
        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0), 1);

        plan.iter()
            .map(|ins| digger.dig(&mut lagoon, ins))
            .sum::<usize>();
        lagoon
    }

    fn load_plan() -> Result<DigPlan, Rc<str>> {
        let p = std::fs::read_to_string(format!(
            "./src/bin/day18/{}",
            std::env::args()
                .skip(3)
                .next()
                .unwrap_or("sample.txt".into())
        ))
        .map_err(|e| format!("Cannot load file: Reason \"{:?}\"", e))?
        .parse::<DigPlan>()
        .map_err(|e| format!("Failed to parse Plan: Reason \"{:?}\"", e))?;
        Ok(p)
    }
}
