use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

pub(crate) struct Pattern {
    pub(crate) p: Rc<[String]>,
    pub(crate) t: Rc<[String]>
}

impl Pattern {

    // pub(crate) fn fix_smudge(&mut self) -> Option<&mut Self> {
    //     if let Some((idx,_,smudge)) = Pattern::find_smudge(&self.p).max() {
    //         self.p.iter_mut()
    //             .for_each(|line| unsafe {
    //                 let s= line.as_bytes_mut();
    //                 s[idx+smudge] = s[idx-smudge-1];
    //             });
    //         return Some(self)
    //     } else {
    //         if let Some((idx,_,smudge)) = Pattern::find_smudge(&self.t).max() {
    //             self.p[idx+smudge] = self.p[idx-smudge-1].clone();
    //             return Some(self)
    //         }
    //     }
    //     None
    // }
    fn mirror_count_at_index(s: &str, idx:usize) -> usize {
        let (l, r) = s.split_at(idx);
        let li = l.chars().rev();
        let mut ri = r.chars();
        li.take_while(|lc| ri.next().map(|rc| rc.cmp(lc)) == Some(Ordering::Equal) ).count()
    }
    pub(crate) fn find_smudge(pat: &[String]) -> impl Iterator<Item=(usize, usize, usize)> + '_ {
        let (width, height) = (pat[0].len(), pat.len());
        let mut smudge_counter = vec![0; width];

        (1..pat[0].len())
            .filter_map(move |idx| {
                // println!("idx: {idx}");
                smudge_counter.fill(0);
                let mut radius = 0;
                let line_count = pat.iter()
                    // .inspect(|p| print!("{:?} -> ",(&p,idx)))
                    .map(|line| Pattern::mirror_count_at_index(line, idx))
                    // .inspect(|p| println!("{:?} : ",p)  )
                    .take_while(|&r| {
                        radius = std::cmp::max(r,radius);
                        smudge_counter[r] += 1;
                        smudge_counter[0] < 2 && smudge_counter[..radius].iter().sum::<usize>() < 2
                    })
                    .count();
                // println!("cand: {:?} : ",(idx,radius,line_count, &smudge_counter[..=radius]));

                if line_count == height && smudge_counter[radius] == height-1 {
                    // println!("Got: {:?} : ",(idx,radius,&smudge_counter[..radius]));
                    Some((idx, radius, smudge_counter[..radius].iter().position(|s| 1.eq(s)).unwrap()))
                } else { None }
            })
    }
    fn find_perfect_mirror(pat: &[String]) -> impl Iterator<Item=(usize, usize)> + '_ {
        let (width, height) = (pat[0].len(), pat.len());

        (1..pat[0].len())
            .filter_map(move |idx| {
                // println!("idx: {idx}");
                let mut radius = usize::MAX;

                let line_count = pat.iter()
                    // .inspect(|p| print!("{:?} -> ",&p))
                    .map(|line| Pattern::mirror_count_at_index(line, idx))
                    .take_while(|&r| {
                        radius = std::cmp::min(r,radius);
                        idx+radius == width || idx-radius == 0
                    })
                    // .inspect(|p| println!("{:?} : ",(idx,p))  )
                    .count();

                if line_count == height {
                    Some((idx, radius))
                } else { None }
            })
    }

    pub(crate) fn find_horizontal_mirror_max(&self) -> Option<(usize, usize)> {
        Pattern::find_perfect_mirror(&self.p)
            // .inspect(|p| println!("Sum{:?} -> ",&p))
            .max_by_key(|p| p.1)
    }
    pub(crate) fn find_vertical_mirror_max(&self) -> Option<(usize, usize)> {
        Pattern::find_perfect_mirror(&self.t)
            // .inspect(|p| println!("Sum{:?} -> ",&p))
            .max_by_key(|p| p.1)
    }
}
impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.lines().map(|s| String::from(s)).collect::<Rc<[String]>>();
        let t =
            (0..p[0].len())
                .map(|col| {
                    p.iter().map(|line| line.chars().skip(col).next().unwrap()).collect::<String>()
                })
                .collect::<Rc<[String]>>();

        Ok(Pattern { p, t })
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .finish_non_exhaustive()?;
        f.write_str("\n")?;
        self.p.iter()
            .for_each(|line| {
                f.write_fmt(format_args!("{:2?}\n",line)).expect("ops")
            });
        Ok(())
    }
}