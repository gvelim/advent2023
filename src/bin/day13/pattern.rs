use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

pub(crate) struct Pattern {
    pub(crate) p: Rc<[String]>,
    pub(crate) t: Rc<[String]>
}

impl Pattern {

    fn reflections_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }

    pub(crate) fn find_smudged_reflection(pat: &[String]) -> impl Iterator<Item=(usize, usize, usize)> + '_ {
        let (width, height) = (pat[0].len(), pat.len());
        let mut smudge_counter = vec![0; width];

        (1..pat[0].len())
            .filter_map(move |idx| {
                let mut radius = 0;
                smudge_counter.fill(0);

                let line_found = pat.iter()
                    .map(|line| Pattern::reflections_at_index(line, idx))
                    .all(|r| {
                        radius = std::cmp::max(r,radius);
                        smudge_counter[r] += 1;
                        smudge_counter[0] < 2 && smudge_counter[..radius].iter().sum::<usize>() < 2
                    });

                if line_found && smudge_counter[radius] == height-1 {
                    Some((
                        idx, radius,
                        smudge_counter[..radius].iter().position(|s| 1.eq(s)).unwrap()
                    ))
                } else { None }
            })
    }

    fn find_perfect_reflection(pat: &[String]) -> impl Iterator<Item=(usize, usize)> + '_ {
        let width = pat[0].len();

        (1..pat[0].len())
            .filter_map(move |idx| {
                let mut radius = usize::MAX;

                if pat.iter()
                    .map(|line| {
                        radius = std::cmp::min(Pattern::reflections_at_index(line, idx), radius);
                        radius
                    })
                    .all(|r| idx+r == width || idx-r == 0 )
                {
                    Some((idx, radius))
                } else {
                    None
                }
            })
    }

    pub(crate) fn find_horizontal_mirror(&self) -> Option<(usize, usize)> {
        Pattern::find_perfect_reflection(&self.p).next()
    }
    pub(crate) fn find_vertical_mirror(&self) -> Option<(usize, usize)> {
        Pattern::find_perfect_reflection(&self.t).next()
    }
}
impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.lines().map(String::from).collect::<Rc<[String]>>();
        let t =
            (0..p[0].len())
                .map(|col| {
                    p.iter().map(|line| line.chars().nth(col).unwrap()).collect::<String>()
                })
                .collect::<Rc<[String]>>();

        Ok(Pattern { p, t })
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Pattern\n")?;
        self.p.iter()
            .for_each(|line| {
                f.write_fmt(format_args!("{:?}\n",line)).expect("ops")
            });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::valley::Valley;

    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(
            valley.patterns[0].find_vertical_mirror(),
            None
        );
        assert_eq!(
            valley.patterns[1].find_vertical_mirror(),
            Some((4, 3))
        );
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(
            valley.patterns[0].find_horizontal_mirror(),
            Some((5,4))
        );
        assert_eq!(
            valley.patterns[1].find_horizontal_mirror(),
            None
        );
    }
}