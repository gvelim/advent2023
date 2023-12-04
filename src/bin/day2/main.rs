use std::collections::HashMap;
use std::str::FromStr;
use crate::Colour::Red;

fn main() {
    let input = std::fs::read_to_string("src/bin/day2/input.txt").unwrap_or_else(|e| panic!("{e}"));
    let arr = input
        .lines()
        .map(|game| Game::from_str(game).expect("Ops!"))
        .collect::<Vec<_>>();

    arr.iter().for_each(|g| println!("{:?}",g) );

}

#[derive(Debug,Eq, PartialEq,Hash)]
enum Colour { Red, Green, Blue }
#[derive(Debug)]
struct Run {
    picked: HashMap<Colour,u32>
}
impl FromStr for Run {
    type Err = ();

    /// convert " 3 blue, 4 red"," 1 red, 2 green, 6 blue", "2 green"
    /// to [(Blue,3),(Red,4)], etc
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let balls = input
            .trim()
            .split(',')
            .map(|balls| {
                let mut split = balls.trim().split(' ');
                let count = u32::from_str_radix(split.next().unwrap(), 10).expect("Ops!");
                let colour = match split.next().unwrap().trim() {
                    "red" => Colour::Red,
                    "green" => Colour::Green,
                    "blue" => Colour::Blue,
                    err => {println!("What's this \"{err}\"?!"); unreachable!("Shouldn't be here")}
                };
                (colour,count)
            })
            .collect::<HashMap<_, _>>();
        Ok(Run { picked: balls })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    runs: Vec<Run>
}

impl FromStr for Game {
    type Err = ();

    /// should parse the string "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    /// Game { 1, [ {(Blue,3),(Red,4)},{(Red,1),(Green,2),(Blue,6)},{(Green,2)} ]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut gsplit = input.split(':');
        let id = u32::from_str_radix(gsplit.next().unwrap().split(' ').last().unwrap(), 10).expect("Ops");
        let runs = gsplit
            .next().unwrap()
            .split(';')
            .map(|run| Run::from_str(run).expect("Ops!"))
            .collect::<Vec<_>>();
        Ok(Game {id, runs})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT : &str =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

    #[test]
    fn test_parse_input() {
        let arr = Game::from_str(INPUT).expect("Ops!");
        arr.iter().for_each(|g| println!("{:?} = {}",g, g.sum()) );
        assert!(true);
    }

    #[test]
    fn test_run_parse() {
        let input = [" 3 blue, 4 red"," 1 red, 2 green, 6 blue"," 2 green"];
        input.iter()
            .for_each(|inp| {
                let run = Run::from_str(inp).expect("Ops!");
                println!("Run: {:?}",run);
            });
    }

}