
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub(crate) enum Directions { Left, Right }
impl Directions {
    pub(crate) fn parse(s: &str) -> impl Iterator<Item=Self> + '_ {
        s.chars()
            .map(|c| match c {
                'R' => Directions::Right,
                'L' => Directions::Left,
                _ => unreachable!()
            })
            .cycle()
    }
}
