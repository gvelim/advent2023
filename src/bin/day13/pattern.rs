use std::cmp::Ordering;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) struct Pattern {
    p: Vec<String>,
    t: Vec<String>,
    pub(crate) max: Option<(usize, usize)>
}

impl Pattern {
    #[inline]
    fn mirror_count_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }
    fn find_mirror(pat: &[String]) -> impl Iterator<Item=(usize,usize)> + '_ {
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
    }
    pub(crate) fn find_horizontal_mirror_max(&self) -> Option<(usize, usize)> {
        Pattern::find_mirror(&self.p)
            .max_by_key(|p| p.1)
    }
    pub(crate) fn find_vertical_mirror_max(&self) -> Option<(usize, usize)> {
        Pattern::find_mirror(&self.t)
            .max_by_key(|p| p.1)
    }
    pub(crate) fn find_horizontal_mirror_min(&self) -> Option<(usize, usize)> {
        Pattern::find_mirror(&self.p)
            .min_by_key(|p| p.1)
    }
    pub(crate) fn find_vertical_mirror_min(&self) -> Option<(usize, usize)> {
        Pattern::find_mirror(&self.t)
            .min_by_key(|p| p.1)
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

        Ok(Pattern { p, t, max:None })
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