use std::cmp::Ordering;
use crate::field::Field;

pub(crate) type Step = (char, (usize, usize));

pub(crate) struct PipeLoop {
    pub(crate) path: Vec<Step>
}

impl PipeLoop {
    pub(crate) fn len(&self) -> usize {
        self.path.len()
    }
    pub(crate) fn order_by_scan_lines(&mut self) -> impl Iterator<Item=&mut [Step]> + '_ {
        self.path.sort_by(|(_, a), (_, b)|
            match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                cmp => cmp
            });

        self.path.chunk_by_mut(|(_, a), (_, b)| a.1 == b.1)
    }
}

pub(crate) trait PipeLoopCutter {
    type Output;
    fn get_valid_pairs(&mut self, f: &Field) -> impl Iterator<Item=&Self::Output>;
}

impl PipeLoopCutter for [Step] {
    type Output = Step;

    fn get_valid_pairs(&mut self, f: &Field) -> impl Iterator<Item=&Self::Output> {
        let mut pipes_removed = 0;

        self.iter_mut()
            // clean up needs to be done before we extract the pipe pairs
            .filter_map(move |p| {
                match p.0 {
                    // Remove '-' as we don't need horizontal pipes & count as removed,
                    '-' => { pipes_removed += 1; None },
                    // Remove 'J' from cases like 'FJ' or 'F--J' as 'J' is outer wall, & count as removed
                    'J' if f.connects_left_with(p.1)
                        .is_some_and(|c| 'F'.eq(c)) => { pipes_removed += 1; None },
                    // Remove 'L' from cases like 'L7' or 'L--7' as 'L' is outer wall, & count as removed
                    'L' if f.connects_right_with(p.1)
                        .is_some_and(|c| '7'.eq(c)) => { pipes_removed += 1; None },
                    // capture valid pipe and offset `x` by removals count
                    _ => {
                        p.1.0 -= pipes_removed;
                        Some(&*p)
                    }
                }
            })
    }
}
