use std::cmp::max;
use std::{str::FromStr, rc::Rc};
use crate::run::{Run,RunError};

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) id: u32,
    runs: Rc<[Run]>,
    max: Run
}

impl Game {
    pub(crate) fn is_feasible(&self, run: &Run) -> bool {
        self.runs
            .iter()
            .all(|r| r.is_feasible(run) )
    }
    pub(crate) fn power(&self) -> u32 {
        self.max.power()
    }
}

impl FromStr for Game {
    type Err = RunError;

    /// should parse the string "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    /// Game { 1, [ {(Blue,3),(Red,4)},{(Red,1),(Green,2),(Blue,6)},{(Green,2)} ]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Run{mut red, mut blue, mut green} = Run::default();
        let mut gsplit = input.split(':');
        let id = gsplit
            .next().unwrap()
            .split_ascii_whitespace()
            .last().unwrap()
            .parse::<u32>()?;

        gsplit
            .next().unwrap()
            .split(';')
            .map(|run| run.parse::<Run>())
            .inspect(|run| {
                if let Ok(run) = run {
                    red = max(red, run.red);
                    blue = max(blue, run.blue);
                    green = max(green, run.green);
                }
            })
            .collect::<Result<Rc<_>,_>>()
            .map(|runs|
                Game {
                    id, runs,
                    max: Run { red, green, blue },
                }
            )
    }
}
