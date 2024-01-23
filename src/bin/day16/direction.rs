#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum Direction { Up, Left, Down, Right, UpDown, LeftRight }
use Direction as D;

impl Direction {
    pub(crate) fn direction(&self, tile: u8) -> Direction {
        match (tile, self) {
            (b'/', D::Right) => D::Up,
            (b'/', D::Left) => D::Down,
            (b'/', D::Up) => D::Right,
            (b'/', D::Down) => D::Left,
            (b'\\', D::Right) => D::Down,
            (b'\\', D::Left) => D::Up,
            (b'\\', D::Up) => D::Left,
            (b'\\', D::Down) => D::Right,
            (b'-', D::Up | D::Down) => D::LeftRight,
            (b'|' , D::Right | D::Left) => D::UpDown,
            (_,d) => *d
        }
    }
}
