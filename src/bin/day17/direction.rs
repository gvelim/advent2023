// use Direction as D;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub(crate) enum Direction { Up=0, Right, Down, Left }

#[cfg(test)]
mod test {
}