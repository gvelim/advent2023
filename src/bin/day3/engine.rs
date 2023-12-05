use std::ops::{RangeInclusive};
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct PartNumber {
    pub(crate) number: u32,
    pub(crate) pos: RangeInclusive<usize>
}

#[derive(Debug)]
pub(crate) struct EngineSchematic {
    pub(crate) len: usize,
    pub(crate) schematic: String,
    pub(crate) partnums: Vec<PartNumber>,
    pub(crate) symbols: Vec<(usize,char)>
}

impl EngineSchematic {
    pub(crate) fn part_numbers(&self) -> impl Iterator<Item=&PartNumber> + '_ {
        let len = self.len;
        self.partnums.iter()
            .filter(move |pn| {
                self.symbols.iter().any(|(pos,_)| {
                    pn.pos.contains(&(pos + 1))
                        || pn.pos.contains(&(pos - 1))
                        || pn.pos.contains(&(pos + len-1))
                        || pn.pos.contains(&(pos + len))
                        || pn.pos.contains(&(pos + len+1))
                        || pn.pos.contains(&(pos - len+1))
                        || pn.pos.contains(&(pos - len))
                        || pn.pos.contains(&(pos - len-1))
                })
            })
    }
}

impl FromStr for EngineSchematic {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.lines();
        let mut len = 0;
        let schematic = input
                .inspect(|l| { len = l.len(); })
                .flat_map(|d| d.chars())
                .collect::<String>();

        let mut partnums: Vec<PartNumber> = vec![];
        let mut symbols: Vec<(usize, char)> = vec![];
        let mut buf = vec![];

        schematic.char_indices()
            .for_each(|c| {
                match c.1 {
                    '.' => {
                        if !buf.is_empty() {
                            let number = buf.iter().map(|(_,c)| c).collect::<String>();
                            partnums.push(PartNumber {
                                number: u32::from_str_radix(number.as_str(), 10).expect("Ops!"),
                                pos: (buf[0].0 ..= buf.last().expect("Ops").0),
                            });
                            buf.clear();
                        }
                    }
                    '0'..='9' => buf.push(c),
                    _ => symbols.push(c),
                }
            });

        Ok(EngineSchematic { len, schematic, partnums, symbols } )
    }
}