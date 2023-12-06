use std::ops::{RangeInclusive};

#[derive(Debug)]
pub(crate) struct PartNumber {
    pub(crate) number: u32,
    pub(crate) pos: RangeInclusive<usize>
}

impl PartNumber {
    pub(crate) fn is_touching(&self, s: &Symbol, len:usize) -> bool {
        self.pos.contains(&(s.0 + 1))
            || self.pos.contains(&(s.0 - 1))
            || self.pos.contains(&(s.0 + len-1))
            || self.pos.contains(&(s.0 + len))
            || self.pos.contains(&(s.0 + len+1))
            || self.pos.contains(&(s.0 - len+1))
            || self.pos.contains(&(s.0 - len))
            || self.pos.contains(&(s.0 - len-1))
    }

}
#[derive(Debug)]
pub(crate) struct Symbol(
    pub(crate) usize,
    pub(crate) char
);
impl From<(usize,char)> for Symbol {
    fn from(value: (usize, char)) -> Self {
        Symbol(value.0,value.1)
    }
}
