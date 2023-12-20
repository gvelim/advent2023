
#[derive(Debug,Clone)]
pub(crate) struct Galaxy {
    pub(crate) pos: (usize, usize)
}

impl Galaxy {
    fn shift_by(&mut self, delta: (usize,usize)) {
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }
    fn distance_to(&self, dst: Galaxy) -> usize {
        todo!()
    }
}
