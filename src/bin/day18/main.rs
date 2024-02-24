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
    InvalidDirection(String),
    InvalidRunLength(String),
    InvalidRGB(String),
    InvalidFormat(String)
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
            Self::InvalidDirection(s) => write!(f, "Cannot parse Direction. Received: {:?}",s),
            Self::InvalidRunLength(s) => write!(f, "Cannot parse RunLength. Received: {:?}",s),
            Self::InvalidRGB(s) => write!(f, "Cannot parse RGB values. Received: {:?}",s),
            Self::InvalidFormat(s) => write!(f, "Expecting 3 parts. Received: {:?}",s),
        }
    }
}

impl FromStr for Instruction {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        if split.clone().count() != 3 { return Err(InstructionErr::InvalidFormat(format!("{}",s)))}

        let dir = split.next().ok_or(InstructionErr::InvalidDirection(format!("{}",s)))?;
        let run = split.next().ok_or(InstructionErr::InvalidRunLength(format!("{}",s)))?;
        let rgb = split.next().ok_or(InstructionErr::InvalidRGB(format!("{}",s)))?.trim_matches(['(',')','#']);
        if rgb.len() != 6 { return Err(InstructionErr::InvalidRGB(format!("{}",s)));}

        Ok(Instruction {
            dir: match dir {
                "D" => Some(Direction::D),
                "L" => Some(Direction::L),
                "R" => Some(Direction::R),
                "U" => Some(Direction::U),
                _ => None
            }.ok_or(InstructionErr::InvalidDirection(format!("{}",s)))?,
            run: usize::from_str(run).or(Err(InstructionErr::InvalidRunLength(format!("{}",s))))?,
            rgb: (
                u8::from_str_radix(&rgb[..=1],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?,
                u8::from_str_radix(&rgb[2..=3],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?,
                u8::from_str_radix(&rgb[4..=5],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?
            )
        })
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
            ("K 5 (#af8603)", Err(InstructionErr::InvalidDirection("K 5 (#af8603)".to_string()))),
            ("L a (#1a3700)", Err(InstructionErr::InvalidRunLength("L a (#1a3700)".to_string()))),
            ("U 10 (#6534071)", Err(InstructionErr::InvalidRGB("U 10 (#6534071)".to_string()))),
            ("U 10 (#65L071)", Err(InstructionErr::InvalidRGB("U 10 (#65L071)".to_string()))),
            ("U 10 (#G071)", Err(InstructionErr::InvalidRGB("U 10 (#G071)".to_string()))),
            ("U 10 [#4d17d2]", Err(InstructionErr::InvalidRGB("U 10 [#4d17d2]".to_string()))),
            ("U 10 (*4d17d2)", Err(InstructionErr::InvalidRGB("U 10 (*4d17d2)".to_string()))),
            ("U10 (#534071)", Err(InstructionErr::InvalidFormat("U10 (#534071)".to_string()))),
        ];

        for (inp,out) in data {
            let instr = inp.parse::<Instruction>();
            println!("Test => \n\tInput: {:?}, \n\tOutput: {:?}", inp, instr);
            assert_eq!( instr, out);
        }

    }
}