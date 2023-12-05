use std::cmp::max;
use std::str::FromStr;
use crate::run::Run;

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) id: u32,
    runs: Vec<Run>,
    max: Run
}

impl Game {
    pub(crate) fn is_feasible(&self, run: &Run) -> bool {
        let &Run{ red,green,blue} = run;
        self.runs
            .iter()
            .all(|run| run.red <= red && run.blue <= blue && run.green <= green )
    }
    pub(crate) fn power(&self) -> u32 {
        let Run { red, green, blue} = self.max;
        red * green * blue
    }
}

impl FromStr for Game {
    type Err = ();

    /// should parse the string "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    /// Game { 1, [ {(Blue,3),(Red,4)},{(Red,1),(Green,2),(Blue,6)},{(Green,2)} ]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Run{mut red, mut blue, mut green} = Run::default();
        let mut gsplit = input.split(':');
        let id = u32::from_str_radix(gsplit.next().unwrap().split(' ').last().unwrap(), 10).expect("Ops");
        let runs = gsplit
            .next().unwrap()
            .split(';')
            .map(|run| Run::from_str(run).expect("Ops!") )
            .inspect(|run| {
                red = max(red, run.red);
                blue = max(blue, run.blue);
                green = max(green, run.green);
            })
            .collect::<Vec<_>>();

        Ok(Game {
            id, runs,
            max: Run { red, green, blue },
        })
    }
}