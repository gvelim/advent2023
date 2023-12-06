use std::str::FromStr;
use super::parts::*;

#[derive(Debug)]
pub(crate) struct EngineSchematic {
    pub(crate) len: usize,
    pub(crate) partnums: Vec<PartNumber>,
    pub(crate) symbols: Vec<Symbol>
}

impl EngineSchematic {
    pub(crate) fn part_numbers(&self) -> impl Iterator<Item=&PartNumber> + '_ {
        let len = self.len;
        self.partnums.iter()
            .filter(move |pn| {
                self.symbols.iter().any(|s| pn.is_touching(&s,len))
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
        let mut symbols: Vec<Symbol> = vec![];
        let mut buf = vec![];

        let make_part_number = |buf: &Vec<(usize, char)>| {
            let number = buf.iter().map(|(_,c)| c).collect::<String>();
            PartNumber {
                number: u32::from_str_radix(number.as_str(), 10).expect("Ops!"),
                pos: (buf[0].0 ..= buf.last().expect("Ops").0),
            }
        };

        schematic.char_indices()
            .for_each(|c| {
                match c.1 {
                    '.' => {
                        if !buf.is_empty() {
                            partnums.push(make_part_number(&buf) );
                            buf.clear();
                        }
                    }
                    '0'..='9' => buf.push(c),
                    _ => {
                        symbols.push(c.into());
                        if !buf.is_empty() {
                            partnums.push(make_part_number(&buf) );
                            buf.clear();
                        }
                    },
                }
            });

        Ok(EngineSchematic { len, partnums, symbols } )
    }
}