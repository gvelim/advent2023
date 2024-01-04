use std::cmp::Ordering;
use std::fmt::Debug;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let valley = Valley::parse(&input);

    let out = valley.patterns.iter()
        .map(|pat| {
            // (pat.find_vertical_mirror(), pat.find_horizontal_mirror())
            pat.find_vertical_mirror()
        })
        .inspect(|p| println!("{:?}",p))
        // .map(|(v,h)| {
        //     v.unwrap_or((0,0)).0 * 100
        //         + h.unwrap_or((0,0)).0
        // })
        .map(|v| {
            v.unwrap_or((0,0)).0 * 100
                // + h.unwrap_or((0,0)).0
        })
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
    fn mirror_count_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }
    fn find_horizontal_mirror(&self) -> Option<(usize,usize)> {
        (1..self.p[0].len())
            .filter_map(|idx| {
                println!("idx: {idx}");
                let mut range = usize::MAX;
                let count = self.p
                    .iter()
                    .inspect(|p| print!("{:?} -> ",&p))
                    .map(|line| Pattern::mirror_count_at_index(line, idx))
                    .take_while(|&c| {
                        range = std::cmp::min(range,c);
                        c != 0
                    })
                    .inspect(|p| println!("{:?}",&p)  )
                    .count();
                if count != self.p.len() { None } else {
                    Some((idx, range))
                }
            })
            .inspect(|p| println!("Sum{:?} -> ",&p))
            .max_by_key(|p| p.1)
    }
    fn find_line_mirror(s: &str) -> Option<(usize, usize)> {
        (1..s.len())
            .map(|idx| {
                (idx,Self::mirror_count_at_index(s, idx))
            })
            // .inspect(|p| println!("{:?}",p))
            .max_by_key(|key| key.1)
    }
    fn find_vertical_mirror(&self) -> Option<(usize,usize)> {
        let transpose = (0 ..self.p[0].len())
            .map(|col| {
                self.p.iter().map(|line| line.chars().skip(col).next().unwrap()).collect::<String>()
            })
            .collect::<Vec<_>>();

        let mut last = None;
        transpose.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|line| {
                Pattern::find_line_mirror(&line)
            })
            .inspect(|p| println!("{:?}",&p)  )
            .all(|a| {
                // if last.is_none() { last = a; true } else {
                //     let ret = last.cmp(&a) == Ordering::Equal;
                    last = a;
                //     ret
                true
            })
            .then(|| last.unwrap())
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
            .map(|(v,h)| {
                v.unwrap_or((0,0)).0 * 100
                    + h.unwrap_or((0,0)).0
            })
            .sum::<usize>();

        assert_eq!(out,405);
    }
    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        assert_eq!(valley.patterns[0].find_vertical_mirror(), Some((4,3)));
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        assert_eq!(valley.patterns[0].find_horizontal_mirror(), Some((4,12)));
    }

    #[test]
    fn test_find_line_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns[0].p.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|line| Pattern::find_line_mirror(line))
            .inspect(|p| println!("{:?}",&p)  )
            .all(|_| true);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }


}