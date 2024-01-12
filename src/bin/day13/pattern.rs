use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

pub(crate) type Reflection = usize;

pub(crate) struct Pattern {
    pub(crate) p: Rc<[String]>,
    pub(crate) t: Rc<[String]>
}

impl Pattern {

    fn reflections_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.bytes().rev();
        let mut ri = r.bytes();
        li.take_while(|&lc| ri.next() == Some(lc)).count()
    }

    pub(crate) fn find_smudged_reflection(pat: &[String]) -> impl Iterator<Item=Reflection> + '_ {
        let (width, height) = (pat[0].len(), pat.len());
        let mut smudge_counter = vec![0; width];

        (1..width)
            .filter_map(move |idx| {
                let mut radius = usize::MIN;
                smudge_counter.fill(0);

                let line_found = pat.iter()
                    .map(|line| Pattern::reflections_at_index(line, idx))
                    .all(|r| {
                        radius = std::cmp::max(r,radius);
                        smudge_counter[r] += 1;
                        smudge_counter[0] < 2 //&& smudge_counter[..radius].iter().sum::<usize>() < 2
                    });

                if line_found && smudge_counter[radius] == height-1 {
                    Some(idx)
                } else { None }
            })
    }

    pub(crate) fn find_perfect_reflection(pat: &[String]) -> impl Iterator<Item=Reflection> + '_ {
        let width = pat[0].len();

        (1..width)
            .filter_map(move |idx|
                pat.iter()
                    .map(|line| Pattern::reflections_at_index(line, idx))
                    .all(|r| idx == r || idx + r == width)
                    .then(|| idx)
            )
    }
    fn transpose(p: &[String]) -> impl Iterator<Item=String> + '_ {
        (0..p[0].len())
            .map(move |col| {
                p.iter().map(|line| line.as_bytes()[col] as char).collect::<String>()
            })
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.lines().map(String::from).collect::<Rc<[String]>>();
        let t = Pattern::transpose(&p).collect::<Rc<[String]>>();

        Ok(Pattern { p, t })
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut piter = self.p.iter();
        let mut titer = self.t.iter();
        let lines = if self.p.len() > self.p[0].len() { self.p.len() } else { self.p[0].len() };

        f.write_str("Pattern\n")?;

            for _ in 0..lines {
                piter.next().map(|line| {
                    line.chars().for_each(|c| f.write_str(&format!("{:2}",c)).expect("ops"));
                }).or_else(||{ 
                    f.write_str(&format!("{:1$}",' ',self.p[0].len()*2)).expect("msg");
                    None
                });
                f.write_str("  -->  ")?;
                titer.next().map(|line| {
                    line.chars().for_each(|c| f.write_str(&format!("{:2}",c)).expect("ops"));
                });
                f.write_str("\n")?
            };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::pattern::Pattern;
    use crate::valley::Valley;

    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(
            Pattern::find_perfect_reflection(&valley.patterns[0].t).next(),
            None
        );
        assert_eq!(
            Pattern::find_perfect_reflection(&valley.patterns[1].t).next(),
            Some(4)
        );
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(
            Pattern::find_perfect_reflection(&valley.patterns[0].p).next(),
            Some(5)
        );
        assert_eq!(
            Pattern::find_perfect_reflection(&valley.patterns[1].p).next(),
            None
        );
    }
}