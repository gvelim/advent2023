
#[derive(Debug,Clone,PartialEq)]
pub(crate) struct Galaxy {
    pub(crate) pos: (usize, usize)
}

impl Galaxy {
    pub(crate) fn shift_by(&mut self, delta: (usize, usize)) {
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }
    pub(crate) fn distance_to(&self, dst: &Galaxy) -> usize {
        // Using the Manhattan distance formula
        dst.pos.0.abs_diff(self.pos.0) + dst.pos.1.abs_diff(self.pos.1)
    }
}
