// use Direction as D;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub(crate) enum Direction { Up=0, Right, Down, Left }

impl Direction {
    pub(crate) fn directions(&self) -> impl Iterator<Item=Direction> + 'static {
        use Direction as D;
        match self{
            &dir@(D::Up|D::Down) => [dir, D::Left, D::Right],
            &dir@(D::Right|D::Left) => [dir, D::Up, D::Down],
        }.into_iter()
    }
}

#[cfg(test)]
mod test {
}