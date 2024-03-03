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
    depth: Depth
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
        self.1.0
    }
    fn g(&self) -> u8 {
        self.1.1
    }
    fn b(&self) -> u8 {
        self.1.2
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

    fn floodfill_intersections(&self, line: Unit) -> impl Iterator<Item=(Unit,Unit)> + '_ {
        use Direction as D;
        let mut last: Option<(&Unit, &Direction)> = None;

        self.map
            .range(Position(Unit::MIN, line)..=Position(Unit::MAX, line))
            .filter_map(move |(Position(x,_), Trench(.., d))| {
                let mut out = None;
                if let Some((lx,ld)) = last {
                    out = match (ld, d) {
                        (D::U, D::D) |
                        (D::U, D::L) |
                        (D::R, D::D) |
                        (D::R, D::L) => Some((*lx,*x)),
                        _ => None,
                    }
                }
                last = Some((x,d));
                out
            })
    }

    pub(crate) fn calculate_area(&self) -> usize {
        (self.min.1..=self.max.1)
            .flat_map(|y|
                self.floodfill_intersections(y)
                    .map(|(x1, x2)| (x2 - x1 - 1) as usize)
            )
            .sum::<usize>()
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use colored::*;
        use Direction as D;

        writeln!(f, "Lagoon")?;
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                write!(f, "{:2}", &self.map
                        .get(&Position(x, y))
                        .map(|t| match t.2 {
                            D::U => "↑",
                            D::R => "→",
                            D::D => "↓",
                            D::L => "←",
                        }.truecolor(t.r(), t.g(), t.b()))
                        .unwrap_or(".".into())
                )?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use std::rc::Rc;

    use super::*;
    use crate::digging_plan::DigPlan;

    #[test]
    fn test_lagoon_area() {
        let test_data = [
            ("sample.txt",62),
            ("sample1.txt",87),
            ("sample2.txt",119),
            ("sample3.txt",157),
        ];

        for (f,out) in test_data {
            let plan = match load_plan(Some(f.into())) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            };

            let lagoon = dig_lagoon(&plan);
            let area = lagoon.calculate_area();

            println!("{:?}\nTrench     : {}\nLagoon area: {area}\nTotal      : {}",
                lagoon, lagoon.map.len(),lagoon.map.len() + area
            );
            assert_eq!(lagoon.map.len() + area, out)
        }
    }

    #[test]
    fn test_dig_lagoon() {
        let plan = match load_plan(None) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let lagoon = dig_lagoon(&plan);

        println!("Steps: {}\n{:?}", lagoon.map.len(), lagoon);
        assert_eq!(lagoon.map.len(), 38)
    }

    #[test]
    fn test_lagoon_floodfill_intersections() {
        let test_data = [
            ("sample.txt",vec![(0i16, 6i16),(2, 6),(2, 6),(2, 6),(2, 4),(0, 4),(1, 4),(1, 6)]),
            ("sample1.txt",vec![(0, 4),(6, 10),(0, 2),(8, 10),(0, 2),(8, 10),(0, 2),(8, 10),(0, 2),(4, 6),(8, 10),(0, 2),(4, 6),(8, 10),(0, 10)]),
            ("sample2.txt",vec![(0, 4),(8, 12),(0, 4),(8, 12),(0, 4),(8, 12),(0, 4),(8, 12),(0, 12),(0, 4),(8, 12),(0, 4),(8, 12),(0, 4),(8, 12),(0, 4),(8, 12)]),
            ("sample3.txt",vec![(-2, 4),(0, 4),(0, 4),(6, 8),(0, 4),(6, 8),(-9, -5),(0, 8),(-6, -2),(0, 4),(6, 8),(-6, -2),(0, 4),(6, 8),(-4, -2),(0, 4),(-4, 4),(-4, 6),(-6, 6)]),
        ];

        for (f,out) in test_data {
            let plan = std::fs::read_to_string(
                format!("./src/bin/day18/{}",f)
            )
            .expect("Cannot load file")
            .parse::<DigPlan>()
            .expect("Failed to parse Plan");

            let lagoon = dig_lagoon(&plan);
            let mut res = out.iter();

            println!("{:?}",lagoon);

            (lagoon.min.1..=lagoon.max.1)
                .inspect(|y| print!("\nLine {y:2}: "))
                .flat_map(|y| {
                    lagoon.floodfill_intersections(y)
                })
                .inspect(|p| print!("{:?},",p))
                .for_each(|p|{
                    assert_eq!(Some(&p),res.next());
                })
        }
    }

    fn dig_lagoon(plan: &DigPlan) -> Lagoon {
        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0), 1);

        plan.iter()
            .map(|ins| digger.dig(&mut lagoon, ins))
            .sum::<usize>();
        lagoon
    }

    pub fn load_plan(file: Option<String>) -> Result<DigPlan, Rc<str>> {
        let p = std::fs::read_to_string(format!(
            "./src/bin/day18/{}",
            std::env::args()
                .skip(3)
                .next()
                .unwrap_or(file.unwrap_or("sample.txt".into()))
        ))
        .map_err(|e| format!("Cannot load file: Reason \"{:?}\"", e))?
        .parse::<DigPlan>()
        .map_err(|e| format!("Failed to parse Plan: Reason \"{:?}\"", e))?;
        Ok(p)
    }
}
