use crate::{
    field::Field,
    direction::{
        Direction,
        Direction::{Down, Left, Right, Up}
    },
    pipeloop::{
        PipeLoop,
        Step
    }
};

#[derive(Debug,PartialEq)]
pub(crate) struct Elf<'a> {
    pub(crate) field: &'a Field,
    pub(crate) pos: (usize, usize),
    pub(crate) dir: Direction,
}

impl Elf<'_> {
    pub(crate) fn traverse_pipes(&mut self, finish:char) -> PipeLoop {
        let mut path = self.take_while(|(p, _)| finish.ne(p)).collect::<Vec<_>>();
        path.push(('S', self.field.start));
        PipeLoop { path }
    }
    pub(crate) fn valid_directions(&self) -> Vec<Direction> {
        [
            self.field.get_pipe((self.pos.0-1,self.pos.1)).and_then(|p| Left.pipe_exit(p)),
            self.field.get_pipe((self.pos.0+1,self.pos.1)).and_then(|p| Right.pipe_exit(p)),
            self.field.get_pipe((self.pos.0,self.pos.1-1)).and_then(|p| Up.pipe_exit(p)),
            self.field.get_pipe((self.pos.0,self.pos.1+1)).and_then(|p| Down.pipe_exit(p))
        ].iter()
            .filter_map(|&d| d)
            .collect::<Vec<_>>()
    }
}

impl Iterator for Elf<'_> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.dir {
            Up => (self.pos.0, self.pos.1-1),
            Right => (self.pos.0+1, self.pos.1),
            Down => (self.pos.0, self.pos.1+1),
            Left => (self.pos.0-1, self.pos.1),
        };
        // have we landed on a valid position ?
        self.field.get_pipe(pos)
            .and_then(|p|
                // Can we enter the new pipe from current direction ?
                self.dir.pipe_exit(p)
                    // new pipe is connected to current hence move one step
                    .map(|dir| {
                        self.pos = pos;
                        self.dir = dir;
                        (p,pos)
                    })
            )
    }
}
