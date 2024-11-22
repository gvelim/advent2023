use std::fmt::Display;
use std::str::FromStr;
use std::rc::Rc;
use super::parts::*;

#[derive(Debug)]
pub(crate) struct EngineSchematic {
    pub(crate) len: usize,
    pub(crate) partnums: Rc<[PartNumber]>,
    pub(crate) symbols: Rc<[Symbol]>
}

impl EngineSchematic {
    pub(crate) fn part_numbers(&self) -> impl Iterator<Item=&PartNumber> + '_ {
        let len = self.len;
        self.partnums.iter()
            .filter(move |pn| {
                self.symbols.iter().any(|s| pn.is_touching(s,len))
            })
    }
    pub(crate) fn get_gears_part_numbers(&self, gear: char) -> impl Iterator<Item=Rc<[&PartNumber]>> {
        self.symbols
            .iter()
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
                    .collect::<Rc<_>>();

                // return pairs otherwise skip what was found for this gear
                if pns.len() > 1 { Some(pns) } else { None }
            })
    }
}

#[derive(Debug,PartialEq)]
pub enum ErrorEngineSchematic {
    PartNumberTooLarge,
    ParsedEmptyInput
}

impl Display for ErrorEngineSchematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorEngineSchematic::PartNumberTooLarge => writeln!(f, "PartNumber found exceeds 32bit size"),
            ErrorEngineSchematic::ParsedEmptyInput => writeln!(f, "Parsed input potentialy empty"),
        }
    }
}

impl FromStr for EngineSchematic {
    type Err = ErrorEngineSchematic;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let len = input.lines()
            .next()
            .ok_or(ErrorEngineSchematic::ParsedEmptyInput)?
            .len();
        let schematic = input
            .lines()
            .flat_map(|d| d.chars())
            .collect::<String>();

        // converts a tuple array to a Partnumber
        // e.g. (23,"1"),(24,"4"),(25,"6") => PartNumber { 146, (23..=25) }
        let make_part_number = |buf: &[(usize, char)]| {
            let (rng, number):(Vec<usize>, String) = buf
                .iter()
                .cloned()
                .unzip();

            Ok(PartNumber {
                number: number.parse::<u32>().map_err(|_| ErrorEngineSchematic::PartNumberTooLarge)?,
                pos: (rng[0] ..= rng[rng.len()-1]),
            })
        };

        // We parse both partnumbers & symbols in one pass along with their **positions**
        // the for loop scans (pos, char) tuples
        // converts tuple **sequences** that contain 0..9 chars into PartNumber { Number & range }
        // e.g. (23,"1"),(24,"4"),(25,"6") => PartNumber { 146, (23..=25) }
        let mut partnums: Vec<PartNumber> = vec![];
        let mut symbols: Vec<Symbol> = vec![];
        let mut buf = Vec::with_capacity(40);
        for c in schematic.char_indices() {
            match c.1 {
                // Ignore '.' unless it is preceeded by a partnumbers
                // case 123..
                '.' => {
                    if !buf.is_empty() {
                        partnums.push( make_part_number(&buf)? );
                        buf.clear();
                    }
                }
                // capture partnumber digit
                '0'..='9' => buf.push(c),
                // it should be a symbol if not a digit or '.' hence capture the
                _ => {
                    symbols.push(c.into());
                    // carefull of case ..123*..
                    if !buf.is_empty() {
                        partnums.push( make_part_number(&buf)? );
                        buf.clear();
                    }
                },
            }
        }

        Ok(EngineSchematic { len, partnums: partnums.into(), symbols: symbols.into() } )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str =
        "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    #[test]
    fn test_parse_engine_schematic() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");
        println!("{:?}",es);
    }

    #[test]
    fn test_parse_int_error() {
        let dataset: [(&str,ErrorEngineSchematic);2] = [
            ("9999999999\n...*.......\n..35..633..\n.664.598...", ErrorEngineSchematic::PartNumberTooLarge),
            ("", ErrorEngineSchematic::ParsedEmptyInput)
        ];

        for (test,err) in dataset {
            match test.parse::<EngineSchematic>() {
                Ok(r) => panic!("Received Ok({:?}) instead of Err",r),
                Err(e) => {
                    println!("Error: {} in {:?}",e, test);
                    assert_eq!(e, err)
                },
            }
        }
    }

    #[test]
    fn test_engine_extract_part_numbers() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");
        println!("{:?}\n{:?}",es.partnums,es.symbols);
        let sum = es.part_numbers()
            .inspect(|pn| print!("F::{:?}", pn))
            .map(|pn| pn.number)
            .sum::<u32>();

        assert_eq!(sum,4361)
    }

    #[test]
    fn test_engine_extract_with_symbol() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");

        let sum = es.get_gears_part_numbers('*')
            .inspect(|d| println!("{:?},",d))
            .map(|d| d.iter().map(|d| d.number).product::<u32>())
            .sum::<u32>();

        println!("{:?}",sum);
        assert_eq!(467835,sum)

    }

}
