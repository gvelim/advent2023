use crate::instruction::{Direction, Instruction, Rgb};
use crate::position::{Position, Unit};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
struct Trench(Rgb, Direction, Option<Direction>);

impl Trench {
    fn r(&self) -> u8 { self.0.0 }
    fn g(&self) -> u8 { self.0.1 }
    fn b(&self) -> u8 { self.0.2 }
}

pub(crate) struct Digger {
    pos: Position,
    last: Option<Direction>,
}

impl Digger {
    pub fn new(pos: Position) -> Digger {
        Digger { pos, last: None }
    }
    pub fn dig(&mut self, lagoon: &mut Lagoon, instr: &Instruction) -> usize {
        let ret = (0..instr.run)
            .take_while(|_| {
                lagoon
                    .dig_trench(
                        *self.pos.next_mut(instr.dir),
                        Trench(instr.rgb, instr.dir, self.last),
                    )
                    .is_none()
            })
            .count();
        self.last = Some(instr.dir);
        ret
    }
}

pub(crate) struct Lagoon {
    min: Position,
    max: Position,
    map: BTreeMap<Position, Trench>
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

    fn floodfill_intersections(&self, line: Unit) -> impl Iterator<Item = Range<Unit>> + '_ {
        use Direction as D;
        let mut last: Option<(&Unit, &Direction)> = None;

        self.map
            .range(Position(Unit::MIN, line)..=Position(Unit::MAX, line))
            .filter_map(move |(Position(x, _), Trench(_, d, pd))| {
                let mut out = None;
                if let Some((lx, ld)) = last {
                    out = match (ld, d) {
                        (D::U, D::D) |
                        (D::U, D::L) |
                        (D::R, D::D) => Some(*lx..*x),
                        (D::R, D::L)
                            if pd.map(|pd| d.is_clockwise(pd)).unwrap_or(false)
                                => Some(*lx..*x),
                        _ => None,
                    }
                }
                last = Some((x, d));
                out
            })
    }

    pub(crate) fn calculate_area(&self) -> usize {
        (self.min.1..=self.max.1)
            .into_par_iter()
            .map(|y| {
                self.floodfill_intersections(y)
                    .map(|rng| (rng.len() - 1) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

impl Debug for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use colored::*;
        use Direction as D;

        writeln!(f, "Lagoon")?;
        for y in self.min.1..=self.max.1 {
            let mut filler = self.floodfill_intersections(y);
            let mut fill = filler.next();
            for x in self.min.0..=self.max.0 {
                write!(f, "{:2}", &self.map
                        .get(&Position(x, y))
                        .map(|t| match t.1 {
                            D::U => "↑",
                            D::R => "→",
                            D::D => "↓",
                            D::L => "←",
                        }
                        .truecolor(t.r(), t.g(), t.b()))
                        .unwrap_or(if let Some(rng) = &fill {
                            rng.contains(&x)
                                .then_some("◼".truecolor(96, 96, 96))
                                .unwrap_or({
                                    x.eq(&rng.end).then(|| fill = filler.next());
                                    ".".into()
                                })
                        } else {
                            ".".into()
                        })
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
    fn test_lagoon_area_rgb() {
        let plan = load_plan(None).expect("Ops");

        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0));

        let t = std::time::Instant::now();
        let total = plan
            .iter()
            .map(|ins|
                digger.dig(&mut lagoon,&ins.decode_rgb())
            )
            .sum::<usize>();

        let area = lagoon.calculate_area();
        println!(
            "\nPart 2:\n\tLagoon Periphery {}\n\tLagoon area = {}\nTotal: {} - {:?}",
            total,
            area,
            total + area,
            t.elapsed()
        );
        assert_eq!(952408144115, total + area);
    }

    #[test]
    fn test_lagoon_area() {
        let test_data = [
            ("sample.txt", 62),
            ("sample1.txt", 87),
            ("sample2.txt", 119),
            ("sample3.txt", 170),
        ];

        for (f, out) in test_data {
            let plan = match load_plan(Some(f.into())) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            };

            let lagoon = dig_lagoon(&plan);
            let area = lagoon.calculate_area();

            println!(
                "{:?}\nTrench     : {}\nLagoon area: {area}\nTotal      : {}",
                lagoon,
                lagoon.map.len(),
                lagoon.map.len() + area
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
        let test_data: [(&str, Vec<Range<Unit>>);4] = [
            ("sample.txt",vec![(0..6),(2..6),(2..6),(2..6),(2..4),(0..4),(1..4),(1..6)]),
            ("sample1.txt",vec![(0..4),(6..10),(0..2),(8..10),(0..2),(8..10),(0..2),(8..10),(0..2),(4..6),(8..10),(0..2),(4..6),(8..10),(0..10)]),
            ("sample2.txt",vec![(0..4),(8..12),(0..4),(8..12),(0..4),(8..12),(0..4),(8..12),(0..12),(0..4),(8..12),(0..4),(8..12),(0..4),(8..12),(0..4),(8..12)]),
            ("sample3.txt",vec![(-2..4),(0..4),(-9..-5),(0..4),(6..8),(-9..-5),(0..4),(6..8),(-9..-2),(0..8),(-6..-2),(0..4),(6..8),(-6..-2),(0..4),(6..8),(-4..-2),(0..4),(-4..4),(-4..6),(-6..6)]),
        ];

        for (f, out) in test_data {
            let plan = match load_plan(Some(f.into())) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            };

            let lagoon = dig_lagoon(&plan);
            let mut res = out.iter();

            println!("{:?}", lagoon);

            (lagoon.min.1..=lagoon.max.1)
                .inspect(|y| print!("\nLine {y:2}: "))
                .flat_map(|y| lagoon.floodfill_intersections(y))
                .inspect(|p| print!("{:?},", p))
                .for_each(|p| {
                    assert_eq!(Some(&p), res.next());
                });
            println!();
        }
    }

    fn dig_lagoon(plan: &DigPlan) -> Lagoon {
        let mut lagoon = Lagoon::default();
        let mut digger = Digger::new(Position(0, 0));

        plan.iter()
            .map(|ins| digger.dig(&mut lagoon, ins))
            .sum::<usize>();
        lagoon
    }

    pub fn load_plan(file: Option<String>) -> Result<DigPlan, Rc<str>> {
        let f = format!(
            "./src/bin/day18/{}",
            file.unwrap_or(
                std::env::args()
                    .find(|s| s.ends_with(".txt"))
                    .unwrap_or("sample.txt".into())
            )
        );
        let p = std::fs::read_to_string(f)
            .map_err(|e| format!("Cannot load file: Reason \"{:?}\"", e))?
            .parse::<DigPlan>()
            .map_err(|e| format!("Failed to parse Plan: Reason \"{:?}\"", e))?;
        Ok(p)
    }
}
