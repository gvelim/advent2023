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
    fn test_find_mirror() {

        fn find_middle(s: &str, m: usize) -> Option<(usize,usize)> {

            let (l, r) = s.split_at(m);
        
            let mut li = l.chars().rev();
            let mut ri = r.chars();

            println!("-> {:?} ",(l,r));

            let mut count = 0;
            loop {
                match (li.next(),ri.next()) {
                    (Some(lc), Some(rc)) => {
                        print!("\t({:?})",(lc,rc));
                        if lc != rc {
                            return find_middle(s, m+1)
                        } else {
                            count += 1;
                        }
                    },
                    (Some(_), None) => break Some((m,count)),
                    (None, Some(_)) => break Some((m,count)),
                    (None, None) => break Some((m,count)),
                }
            }
        }

        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley
            .patterns[0].p
            .iter()
            .for_each(|line| {
                println!("{:?} - {:?}",line,find_middle(line, line.len()>>1))
            });
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = Valley::parse(&input);

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }


}