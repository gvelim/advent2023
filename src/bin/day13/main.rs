use std::cmp::Ordering;
use std::fmt::Debug;

fn main() {

}

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
    fn find_horizontal_mirror(&self) -> Option<(usize,usize)> {
        let mut last = None;
        self.p
            .windows(2)
            // .inspect(|p| print!("{:?} -> ",&p))
            .map(|line| (Pattern::line_middle(line[0]), Pattern::line_middle(line[1])))
            // .inspect(|p| println!("{:?}",&p)  )
            .all(|(a,b)| {
                last = a;
                a.cmp(&b) == Ordering::Equal
            })
            .then(|| last.unwrap())
    }
    fn line_middle(s: &str) -> Option<(usize, usize)> {
        (1..s.len())
            .map(|idx| {
                let (l, r) = s.split_at(idx);
                let li = l.chars().rev();
                let mut ri = r.chars();
                (idx, li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count())
            })
            .max_by_key(|key| key.1)
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
    fn test_find_horizontal_mirror() {

        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        println!("{:?}, {:?}",&valley.patterns[0], valley.patterns[0].find_horizontal_mirror());
        println!("{:?}, {:?}",&valley.patterns[1], valley.patterns[1].find_horizontal_mirror());
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }


}