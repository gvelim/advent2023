#![feature(iter_collect_into)]
extern crate core;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let valley = input.parse::<Valley>().expect("Ops!");

    let t = std::time::Instant::now();
    let out = valley.patterns.iter()
        .map(|pat| {
            (pat.find_vertical_mirror(), pat.find_horizontal_mirror())
        })
        // .inspect(|p| print!("{:?} -> ",&p))
        .map(|(v,h)| {
            match (v,h) {
                (Some(v), Some(h)) => if v.1 >= h.1 { v.0 * 100 } else { h.0 },
                (Some(v), None) => v.0 * 100,
                (None, Some(h)) => h.0,
                (None,None) => 0
            }
            // v.unwrap_or((0, 0)).0 * 100 + h.unwrap_or((0, 0)).0
        })
        // .inspect(|p| println!("{:?}",&p))
        .sum::<usize>();

    println!("Part 1 : {:?} - {:?}",out, t.elapsed());

}

#[derive(Debug)]
struct Valley {
    patterns: Vec<Pattern>
}
impl FromStr for Valley {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Valley {
            patterns: s.split("\n\n")
                .map(|pat| pat.parse::<Pattern>().expect("Ops!"))
                .collect::<Vec<_>>()
        })
    }
}


struct Pattern {
    p: Vec<String>,
    t: Vec<String>
}

impl Pattern {
    #[inline]
    fn mirror_count_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }
    fn find_mirror(pat: &[String]) -> Option<(usize, usize)> {
        (1..pat[0].len())
            .filter_map(|idx| {
                // println!("idx: {idx}");
                let mut range = usize::MAX;
                let count = pat.iter()
                    // .inspect(|p| print!("{:?} -> ",&p))
                    .map(|line| Pattern::mirror_count_at_index(line, idx))
                    .take_while(|&c| {
                        range = std::cmp::min(range,c);
                        c > 0
                    })
                    // .inspect(|p| println!("{:?} : ",&p)  )
                    .count();
                if count != pat.len() { None } else {
                    if idx+range == pat[0].len() || idx-range < 1 {
                        Some((idx, range))
                    } else { None }
                }
            })
            // .inspect(|p| println!("Sum{:?} -> ",&p))
            .max_by_key(|p| p.1)
    }
    fn find_horizontal_mirror(&self) -> Option<(usize, usize)> {
        Pattern::find_mirror(&self.p)
    }
    fn find_vertical_mirror(&self) -> Option<(usize,usize)> {
        Pattern::find_mirror(&self.t)
    }
}
impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.lines().map(|s| String::from(s)).collect::<Vec<String>>();
        let t =
            (0..p[0].len())
                .map(|col| {
                    p.iter().map(|line| line.chars().skip(col).next().unwrap()).collect::<String>()
                })
                .collect::<Vec<_>>();

        Ok(Pattern { p, t })
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.p.iter()
            .for_each(|line| {
                f.write_fmt(format_args!("{:?}\n",line)).expect("ops")
            });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_sample_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        let out = valley.patterns.iter()
            .map(|pat| {
                (pat.find_vertical_mirror(), pat.find_horizontal_mirror())
            })
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|(v,h)| {
                match (v,h) {
                    (Some(v), Some(h)) => if v.1 > h.1 { v.0 * 100 } else { h.0 },
                    (Some(v), None) => v.0 * 100,
                    (None, Some(h)) => h.0,
                    (None,None) => 0
                }
            })
            .inspect(|p| println!("{:?}",&p))
            .sum::<usize>();

        assert_eq!(out,405);
    }
    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_vertical_mirror())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_horizontal_mirror())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }


}