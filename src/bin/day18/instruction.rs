use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug,PartialEq, Copy, Clone)]
pub(crate) enum Direction { U = 0, R, D, L }

const TURNS: [Direction; 6] = [
    Direction::L,
    Direction::U,
    Direction::R,
    Direction::D,
    Direction::L,
    Direction::U,
];

impl Direction {
    pub fn is_clockwise(&self, last: Direction) -> bool {
        TURNS[*self as usize] == last
    }
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Debug for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct Instruction {
    pub dir: Direction,
    pub run: usize,
    pub rgb: Rgb
}

impl Instruction {
    pub(crate) fn decode_rgb(&self) -> Instruction {
        let s = format!("{}", self.rgb);
        Instruction {
            dir: TURNS[usize::from_str(&s[6..=6]).unwrap() + 2],
            run: usize::from_str_radix(&s[1..=5], 16).unwrap(),
            rgb: Rgb(0,0,0)
        }
    }
}

impl FromStr for Instruction {
    type Err = InstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        if split.clone().count() != 3 {
            return Err(InstructionErr::InvalidFormat(s.into()));
        }

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
        write!(f, "{:?} {} ({:?})", self.dir, self.run, self.rgb)?;
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
            Self::InvalidDirection(s) => write!(f, "Cannot parse Direction. Received: {:?}", s),
            Self::InvalidRunLength(s) => write!(f, "Cannot parse RunLength. Received: {:?}", s),
            Self::InvalidRGB(s) => write!(f, "Cannot parse RGB values. Received: {:?}", s),
            Self::InvalidFormat(s) => write!(f, "Expecting 3 parts. Received: {:?}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_decode_rgb() {
        let test_data = [
            ("D 1 #70c710", "R 461937"),
            ("D 1 #0dc571", "D 56407"),
            ("D 1 #5713f0", "R 356671"),
            ("D 1 #d2c081", "D 863240"),
            ("D 1 #59c680", "R 367720"),
            ("D 1 #411b91", "D 266681"),
            ("D 1 #8ceee2", "L 577262"),
            ("D 1 #caa173", "U 829975"),
            ("D 1 #1b58a2", "L 112010"),
            ("D 1 #caa171", "D 829975"),
            ("D 1 #7807d2", "L 491645"),
            ("D 1 #a77fa3", "U 686074"),
            ("D 1 #015232", "L 5411"),
            ("D 1 #7a21e3", "U 500254"),
        ];

        for (i, o) in test_data {
            let d = i
                .parse::<Instruction>()
                .expect("Cannot Parse Instructions")
                .decode_rgb();
            assert_eq!(o, &format!("{:?} {}", d.dir, d.run));
        }
    }

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

        for (inp, out) in data {
            let instr = inp.parse::<Instruction>();
            println!("Test => \n\tInput: {:?}, \n\tOutput: {:?}", inp, instr);
            assert_eq!(instr, out);
        }
    }
}
