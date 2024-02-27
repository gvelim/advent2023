use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::rc::Rc;

#[derive(Debug,PartialEq, Copy, Clone)]
pub(crate) enum Direction { U, R, D, L }

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Rgb(u8,u8,u8);

impl Debug for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0,self.1,self.2)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self,f)
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct Instruction {
    pub dir: Direction,
    pub run: usize,
    pub rgb: Rgb
}

impl FromStr for Instruction {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        if split.clone().count() != 3 { return Err(InstructionErr::InvalidFormat(s.into()))}

        let dir = split.next().ok_or(InstructionErr::InvalidDirection(s.into()))?;
        let run = split.next().ok_or(InstructionErr::InvalidRunLength(s.into()))?;
        let rgb = split.next().ok_or(InstructionErr::InvalidRGB(s.into()))?.trim_matches(['(',')','#']);
        if rgb.len() != 6 { return Err(InstructionErr::InvalidRGB(s.into()));}

        Ok(Instruction {
            dir: match dir {
                "D" => Some(Direction::D),
                "L" => Some(Direction::L),
                "R" => Some(Direction::R),
                "U" => Some(Direction::U),
                _ => None
            }.ok_or(InstructionErr::InvalidDirection(s.into()))?,
            run: usize::from_str(run).or(Err(InstructionErr::InvalidRunLength(s.into())))?,
            rgb: Rgb(
                u8::from_str_radix(&rgb[..=1],16).or(Err(InstructionErr::InvalidRGB(s.into())))?,
                u8::from_str_radix(&rgb[2..=3],16).or(Err(InstructionErr::InvalidRGB(s.into())))?,
                u8::from_str_radix(&rgb[4..=5],16).or(Err(InstructionErr::InvalidRGB(s.into())))?
            )
        })
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} ({:?})",self.dir,self.run,self.rgb)?;
        Ok(())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(PartialEq)]
pub(crate) enum InstructionErr {
    InvalidDirection(Rc<str>),
    InvalidRunLength(Rc<str>),
    InvalidRGB(Rc<str>),
    InvalidFormat(Rc<str>)
}

impl Error for InstructionErr {}

impl Display for InstructionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
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
            ("R 6 (#4d17d2)", Ok(Instruction { dir: Direction::R, run: 6, rgb: Rgb(0x4d, 0x17, 0xD2)})),
            ("U 10 (4d17d2)", Ok(Instruction { dir: Direction::U, run: 10, rgb: Rgb(0x4d, 0x17, 0xD2)})),
            ("U 10 #4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: Rgb(0x4d, 0x17, 0xD2)})),
            ("U 10 4d17d2", Ok(Instruction { dir: Direction::U, run: 10, rgb: Rgb(0x4d, 0x17, 0xD2)})),
            ("K 5 (#af8603)", Err(InstructionErr::InvalidDirection("K 5 (#af8603)".into()))),
            ("L a (#1a3700)", Err(InstructionErr::InvalidRunLength("L a (#1a3700)".into()))),
            ("U 10 (#6534071)", Err(InstructionErr::InvalidRGB("U 10 (#6534071)".into()))),
            ("U 10 (#65L071)", Err(InstructionErr::InvalidRGB("U 10 (#65L071)".into()))),
            ("U 10 (#G071)", Err(InstructionErr::InvalidRGB("U 10 (#G071)".into()))),
            ("U 10 [#4d17d2]", Err(InstructionErr::InvalidRGB("U 10 [#4d17d2]".into()))),
            ("U 10 (*4d17d2)", Err(InstructionErr::InvalidRGB("U 10 (*4d17d2)".into()))),
            ("U10 (#534071)", Err(InstructionErr::InvalidFormat("U10 (#534071)".into()))),
        ];

        for (inp,out) in data {
            let instr = inp.parse::<Instruction>();
            println!("Test => \n\tInput: {:?}, \n\tOutput: {:?}", inp, instr);
            assert_eq!( instr, out);
        }

    }
}
