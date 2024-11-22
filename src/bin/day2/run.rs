use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug,Default,PartialEq)]
pub struct Run {
    pub(crate) red: u32, pub(crate) green: u32, pub(crate) blue: u32
}

impl Run {
    pub fn is_feasible(&self, run: &Run) -> bool {
        self.red <= run.red
        && self.blue <= run.blue
        && self.green <= run.green
    }
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug,PartialEq)]
pub enum RunError {
    InvalidColourValue,
    MissingColourValue,
    InvalidColourName,
    MissingColourName,
}

impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::InvalidColourValue => write!(f, "colour value must be a positive number"),
            RunError::MissingColourValue => write!(f, "colour value not found with colour name"),
            RunError::InvalidColourName => write!(f, "colour name must be one of [ red | green | blue ]"),
            RunError::MissingColourName => write!(f, "colour name not found with colour value"),
        }
    }
}

impl From<ParseIntError> for RunError {
    fn from(_: ParseIntError) -> Self {
        RunError::InvalidColourValue
    }
}

impl FromStr for Run {
    type Err = RunError;

    /// convert " 3 blue, 4 red"," 1 red, 2 green, 6 blue", "2 green"
    /// to [(Blue,3),(Red,4)], etc
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        #[derive(Debug,Eq, PartialEq,Hash)]
        enum Colour { Red, Green, Blue }

        match input
            .trim()
            .split(',')
            .map(|picked| {
                let mut split = picked.trim().split_ascii_whitespace();
                let count = split.next().ok_or(RunError::MissingColourValue)?.parse::<u32>()?;
                let colour = match split.next().ok_or(RunError::MissingColourName)? {
                    "red" => Ok(Colour::Red),
                    "green" => Ok(Colour::Green),
                    "blue" => Ok(Colour::Blue),
                    _ => Err(RunError::InvalidColourName)
                }?;
                Ok((colour,count))
            })
            .collect::<Result<Vec<(Colour,u32)>,RunError>>()
        {
            Ok(colours) => Ok(colours
                .into_iter()
                .fold(Run::default(),|mut run, (col, val)| {
                    match col {
                        Colour::Red => run.red = val,
                        Colour::Green => run.green = val,
                        Colour::Blue => run.blue = val
                    }
                    run
                })
            ),
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_parse() {
        let input = [
            (" 3 blue, 4 red", Run { red: 4, blue: 3, green: 0 }),
            (" 1 red, 2 green, 6 blue", Run { red: 1, blue: 6, green: 2 }),
            (" 2 green", Run { red: 0, blue: 0, green: 2 })
        ];

        for (test, res) in input {
            match test.parse::<Run>() {
                Ok(run) => assert_eq!(run,res),
                Err(e) => panic!("Test {:?} failed with {:?}",test,e)
            }
        }
    }

    #[test]
    fn test_run_parse_errors() {
        let input = [
            (" d blue, 4 red", RunError::InvalidColourValue),
            (" 1 red, 2 green, 6 orange", RunError::InvalidColourName ),
            (" d eno", RunError::InvalidColourValue),
            ("1 , 4 red ", RunError::MissingColourName),
            ("1 4 red ", RunError::InvalidColourName),
            ("red, 4 blue ", RunError::InvalidColourValue)
        ];

        for (test, res) in input {
            match test.parse::<Run>() {
                Ok(_) => panic!("Test {:?} should not succeed",test),
                Err(e) => assert_eq!(e,res, "{:?} expected [{:?}] got [{:?}]",test, res,e)
            }
        }
    }
}
