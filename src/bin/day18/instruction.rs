use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;


#[derive(Debug,PartialEq, Copy, Clone)]
pub(crate) enum Direction { U, R, D, L }

#[derive(PartialEq, Copy, Clone)]
pub struct RGB(u8,u8,u8);

impl Debug for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.0,self.1,self.2)
    }
}
impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self,f)
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct Instruction {
    pub dir: Direction,
    pub run: usize,
    pub rgb: RGB
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
            rgb: RGB(
                u8::from_str_radix(&rgb[..=1],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?,
                u8::from_str_radix(&rgb[2..=3],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?,
                u8::from_str_radix(&rgb[4..=5],16).or(Err(InstructionErr::InvalidRGB(format!("{}",s))))?
            )
        })
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} (#{:02x}{:02x}{:02x})",self.dir,self.run,self.rgb.0,self.rgb.1,self.rgb.2)?;
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}

#[derive(PartialEq)]
pub(crate) enum InstructionErr {
    InvalidDirection(String),
    InvalidRunLength(String),
    InvalidRGB(String),
    InvalidFormat(String)
}

impl Error for InstructionErr {}

impl Display for InstructionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_instruction_parse() {
        let data = [
            ("R 6 (#4d17d2)", Ok(Instruction { dir: Direction::R, run: 6, rgb: RGB(0x4d, 0x17, 0xD2)})),
            ("U 10 (4d17d2)", Ok(Instruction { dir: Direction::U, run: 10, rgb: RGB(0x4d, 0x17, 0xD2)})),
            ("U 10 #4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: RGB(0x4d, 0x17, 0xD2)})),
            ("U 10 4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: RGB(0x4d, 0x17, 0xD2)})),
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