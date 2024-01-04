#![feature(iter_collect_into)]
extern crate core;

use std::cmp::Ordering;
use std::fmt::Debug;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let valley = Valley::parse(&input);

    let out = valley.patterns.iter()
        .map(|pat| {
            (pat.find_vertical_mirror(), pat.find_horizontal_mirror())
        })
        .inspect(|p| print!("{:?} -> ",&p))
        .map(|(v,h)| {
            match (v,h) {
                (Some(v), Some(h)) => if v.1 >= h.1 { v.0 * 100 } else { h.0 },
                (Some(v), None) => v.0 * 100,
                (None, Some(h)) => h.0,
                (None,None) => 0
            }
            // v.unwrap_or((0, 0)).0 * 100 + h.unwrap_or((0, 0)).0
        })
        .inspect(|p| println!("{:?}",&p))
        .sum::<usize>();

    println!("Part 1 : {:?}",out);

}

#[derive(Debug)]
struct Valley<'a> {
    patterns: Vec<Pattern<'a>>
}
impl Valley<'_> {
    fn parse(input: &str) -> Valley {
        Valley {
            patterns: input.split("\n\n")
                .map(|pat| Pattern::from_str(pat))
                .collect::<Vec<_>>()
        }
    }
}


struct Pattern<'a> {
    p: Vec<&'a str>
}

impl<'a> Pattern<'a> {
    #[inline]
    fn mirror_count_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }
    fn find_mirror(pat: &[&str]) -> Option<(usize, usize)> {
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
        let transpose = (0 ..self.p[0].len())
            .map(|col| {
                self.p.iter().map(|line| line.chars().skip(col).next().unwrap()).collect::<String>()
            })
            .collect::<Vec<_>>();

        (1..transpose[0].len())
            .filter_map(|idx| {
                // println!("idx: {idx}");
                let mut range = usize::MAX;
                let count = transpose.iter()
                    // .inspect(|p| print!("{:?} -> ",&p))
                    .map(|line| Pattern::mirror_count_at_index(line, idx))
                    .take_while(|&c| {
                        range = std::cmp::min(range,c);
                        c > 0
                    })
                    // .inspect(|p| println!("{:?}",&p)  )
                    .count();
                if count != transpose.len() { None } else {
                    if idx+range == transpose[0].len() || idx-range < 1 {
                        Some((idx, range))
                    } else { None }
                }
            })
            // .inspect(|p| println!("Sum{:?} -> ",&p))
            .max_by_key(|p| p.1)
    }

    fn from_str(s: &'a str) -> Self {
        Pattern { 
            p: s.lines().collect::<Vec<&str>>()
        }
    }
}

impl<'a> Debug for Pattern<'a> {
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
        let valley = Valley::parse(&input);

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
        let valley = Valley::parse(&input);

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_vertical_mirror())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_horizontal_mirror())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }


}