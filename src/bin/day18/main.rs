use std::{fmt::Debug, str::FromStr};

fn main() {

}

struct InstructionSet {
    set: std::rc::Rc<[Instruction]>
}

impl InstructionSet {
    fn iter(&self) -> impl Iterator<Item = &Instruction> + '_ {
        self.set.iter()
    }
}

impl FromStr for InstructionSet {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = vec![];
        for line in s.lines() {
            set.push( line.parse::<Instruction>()?);
        }
        Ok(InstructionSet{
            set: set.into()
        })
    }
}

#[derive(Debug,PartialEq)]
enum Direction { U, R, D, L}

#[derive(PartialEq)]
struct Instruction {
    dir: Direction, 
    run: usize,
    rgb: (u8,u8,u8)
}

#[derive(PartialEq)]
enum InstructionErr {
    InvalidDirection,
    InvalidRunLength,
    InvalidRGB
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} (#{:02x}{:02x}{:02x})",self.dir,self.run,self.rgb.0,self.rgb.1,self.rgb.2)?;
        Ok(())
    }
}
impl Debug for InstructionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirection => write!(f, "Cannot parse Direction. Check instruction format"),
            Self::InvalidRunLength => write!(f, "Cannot parse RunLength. Check instruction format"),
            Self::InvalidRGB => write!(f, "Cannot parse RGB values. Check instruction format"),
        }
    }
}

impl FromStr for Instruction {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        
        let dir = split.next().ok_or(InstructionErr::InvalidDirection)?;
        let dir = match dir {
                "D" => Some(Direction::D),
                "L" => Some(Direction::L),
                "R" => Some(Direction::R),
                "U" => Some(Direction::U),
                _ => None
            }.ok_or(InstructionErr::InvalidDirection)?;
        
        let run = split.next().ok_or(InstructionErr::InvalidRunLength)?;
        let run = usize::from_str(run).or(Err(InstructionErr::InvalidRunLength))?;
        
        let rgb = split.next().ok_or(InstructionErr::InvalidRGB)?.trim_matches(['(',')','#']);
        if rgb.len() != 6 { return Err(InstructionErr::InvalidRGB);}
        let rgb = (
            u8::from_str_radix(&rgb[..=1],16).or(Err(InstructionErr::InvalidRGB))?,
            u8::from_str_radix(&rgb[2..=3],16).or(Err(InstructionErr::InvalidRGB))?,
            u8::from_str_radix(&rgb[4..=5],16).or(Err(InstructionErr::InvalidRGB))?,
        );

        Ok(Instruction { dir, run, rgb })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instructionset_parse()  {
        let inp = std::fs::read_to_string("./src/bin/day18/sample.txt").expect("Ops!");
        let set = match inp.parse::<InstructionSet>() {
            Ok(set) => set,
            Err(e) => panic!("{:?}",e),
        };
    
        set.iter().for_each(|ins| println!("{:?}",ins) );
        assert_eq!(set.set.len(), 14);
    }

    #[test]
    fn test_instruction_parse() {
        let data = [
            ("R 6 (#4d17d2)", Ok(Instruction { dir: Direction::R, run: 6, rgb: (0x4d,0x17,0xD2)})),
            ("U 10 (4d17d2)", Ok(Instruction { dir: Direction::U, run: 10, rgb: (0x4d,0x17,0xD2)})),
            ("U 10 #4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: (0x4d,0x17,0xD2)})),
            ("U 10 4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: (0x4d,0x17,0xD2)})),
            ("K 5 (#af8603)", Err(InstructionErr::InvalidDirection)),
            ("L a (#1a3700)", Err(InstructionErr::InvalidRunLength)),
            ("U 10 (#6534071)", Err(InstructionErr::InvalidRGB)),
            ("U 10 (#65L071)", Err(InstructionErr::InvalidRGB)),
            ("U 10 (#G071)", Err(InstructionErr::InvalidRGB)),
            ("U 10 [#4d17d2]", Err(InstructionErr::InvalidRGB)),
            ("U 10 (*4d17d2)", Err(InstructionErr::InvalidRGB)),
        ];

        for (inp,out) in data {
            let instr = inp.parse::<Instruction>();
            println!("Test => \n\tInput: {:?}, \n\tOutput: {:?}", inp, instr);
            assert_eq!( instr, out)
        }

    }
}