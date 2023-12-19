
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Direction { Up, Right, Down, Left }

impl Direction {

    // The pipes are arranged in a two-dimensional grid of tiles:
    //
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    //
    // Given (a) a pipe to step on and (b) current direction derive the new direction if valid otherwise None
    pub(crate) fn pipe_exit(&self, c: char) -> Option<Self> {
        use Direction::*;
        match (self, c) {
            (Left|Right , '-') => Some(*self),
            (Up|Down , '|') => Some(*self),
            (Down, 'L') => Some(Right),
            (Left, 'L') => Some(Up),
            (Down, 'J') => Some(Left),
            (Right, 'J') => Some(Up),
            (Up, '7') => Some(Left),
            (Right, '7') => Some(Down),
            (Up, 'F') => Some(Right),
            (Left, 'F') => Some(Down),
            _ => None
        }
    }
}
