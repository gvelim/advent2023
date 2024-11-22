use std::ops::RangeInclusive;

#[derive(Debug)]
pub(crate) struct PartNumber {
    pub(crate) number: u32,
    pub(crate) pos: RangeInclusive<usize>
}

impl PartNumber {
    pub(crate) fn is_touching(&self, s: &Symbol, len:usize) -> bool {
        s.is_touching(self,len)
    }
}

#[derive(Debug)]
pub(crate) struct Symbol(
    pub(crate) usize,
    pub(crate) char
);

impl Symbol {
    pub(crate) fn is_touching(&self, pn: &PartNumber, len:usize) -> bool {
        (self.0 - len-1 ..= self.0 - len+1).contains(pn.pos.end()) ||
            (self.0 - len-1 ..= self.0 - len+1).contains(pn.pos.start()) ||
            (self.0 + len-1 ..= self.0 + len+1).contains(pn.pos.end()) ||
            (self.0 + len-1 ..= self.0 + len+1).contains(pn.pos.start()) ||
            pn.pos.contains(&(self.0 - 1)) ||
            pn.pos.contains(&(self.0 + 1))
    }
}

impl From<(usize,char)> for Symbol {
    fn from(value: (usize, char)) -> Self {
        Symbol(value.0,value.1)
    }
}
