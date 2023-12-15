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
                self.symbols.iter().any(|s| pn.is_touching(s,len))
            })
    }
    pub(crate) fn get_gears_part_numbers(&self, gear: char) -> impl Iterator<Item=Vec<&PartNumber>> {
        self.symbols.iter()
            // only proceed with gear symbol provided
            .filter(move |s| s.1.eq(&gear))
            // .inspect(|d| println!("{:?}",d))
            // per gear symbol
            .filter_map(|s| {
                let pns = self.partnums.iter()
                    // only consider part numbers proximate to the gear
                    // ignore part numbers falling outside the gears reach
                    .filter(|pn| s.is_touching(pn,self.len))
                    // .inspect(|d| println!("{:?}",d))
                    .collect::<Vec<_>>();

                // return pairs otherwise skip what was found for this gear
                if pns.len() > 1 { Some(pns) } else { None }
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
        let mut buf = Vec::with_capacity(40);

        let make_part_number = |buf: &Vec<(usize, char)>| {
            let number = buf.iter().map(|(_,c)| c).collect::<String>();
            PartNumber {
                number: number.parse::<u32>().expect("Ops!"),
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