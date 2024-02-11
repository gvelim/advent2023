// use Direction as D;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Direction { Up=0, Right, Down, Left }

impl Direction {
    pub(crate) fn directions(&self) -> impl Iterator<Item=Direction> + 'static {
        use Direction as D;
        match self{
            D::Up => [D::Up, D::Left, D::Right],
            D::Right => [D::Right, D::Up, D::Down],
            D::Down => [D::Down, D::Left, D::Right],
            D::Left => [D::Left, D::Up, D::Down],
        }.into_iter()
    }
    pub(crate) fn directions_b(&self) -> impl Iterator<Item=Direction> + 'static {
        use Direction as D;
        match self{
            D::Up|D::Down => [D::Left, D::Right],
            D::Left|D::Right => [D::Up, D::Down]
        }.into_iter()
    }

}

#[cfg(test)]
mod test {
}