use std::fmt::{Debug, Formatter, Write};
use std::str::FromStr;

fn main() {

}

#[derive(Default)]
struct ReflectorDish {
    width: usize,
    lines: usize,
    layout: Vec<char>
}

impl FromStr for ReflectorDish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ReflectorDish {
            width: s.lines().next().map(|s| s.len()).unwrap(),
            lines: s.lines().count(),
            layout: s.lines()
                .flat_map(|line| line.chars())
                .collect::<Vec<char>>()
        })
    }
}

impl Debug for ReflectorDish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ReflectorDish\n")?;
        f.write_str(&format!("Width:{}, Length:{}", self.width, self.lines))?;
        for (i,c) in self.layout.iter().enumerate() {
            if i % self.width == 0 {
                f.write_char('\n')?
            };
            f.write_char(' ')?;
            f.write_char(*c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_reflector_dish() {
        let inp = std::fs::read_to_string("src/bin/day14/sample.txt").expect("Ops!");

        let dish = inp.parse::<ReflectorDish>().unwrap_or_default();

        println!("{:?}",dish);

    }

}