use std::{fmt::Debug, num::ParseIntError, str::FromStr};

pub(crate) type Unit = u16;

#[derive(Clone, Copy)]
pub(crate) struct Part {
    // each part is rated in each of four categories
    pub x: Unit, // x: Extremely cool looking
    pub m: Unit, // m: Musical (it makes a noise when you hit it)
    pub a: Unit, // a: Aerodynamic
    pub s: Unit  // s: Shiny
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // x=787,m=2655,a=1222,s=2876
        let Part{x,m,a,s} = self;
        write!(f,"{{x={x},m={m},a={a},s={s}}}")
    }
}

impl Part {
    pub(crate) fn sum(&self) -> Unit {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ParseIntError;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        // {x=787,m=2655,a=1222,s=2876}
        let mut s = inp
            .trim_matches(&['{','}'])
            .split_terminator(',');
        let x = Unit::from_str( &s.next().unwrap()[2..] )?;
        let m = Unit::from_str( &s.next().unwrap()[2..] )?;
        let a = Unit::from_str( &s.next().unwrap()[2..] )?;
        let s = Unit::from_str( &s.next().unwrap()[2..] )?;

        Ok(Part { x, m, a, s })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_parse() {
        let inp = std::fs::read_to_string("src/bin/day19/sample.txt")
            .expect("cannot load sample.txt");
        let data = inp.split("\n\n").skip(1).next().unwrap().lines();

        for inp in data {
            let part = inp.parse::<Part>().expect("Part parsing error!");
            println!("{:?}",part);
            assert_eq!(format!("{:?}",part),inp)
        }
    }

}
