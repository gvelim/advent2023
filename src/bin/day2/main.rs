use std::cmp::max;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("src/bin/day2/input.txt").unwrap_or_else(|e| panic!("{e}"));
    let gref = Game { id: 0, runs: vec![Run { red: 12, blue:14, green:13 }], max: Run { red: 12, blue:14, green:13 }};

    let games = input
        .lines()
        .map(|game| Game::from_str(game).expect("Ops!"))
        .collect::<Vec<_>>();

    let sum = games.iter()
        .filter(|game| game.is_feasible(&gref))
        .map(|game| game.id )
        .sum::<u32>();

    println!("Part 1 : Sum = {sum}");

    let sum = games.iter()
        .map(|game| game.power() )
        .sum::<u32>();

    println!("Part 2 : Sum = {sum}");
}

#[derive(Debug)]
struct Run {
    red: u32, green: u32, blue: u32
}
impl Default for Run {
    fn default() -> Self {
        Run { red: 0, green: 0, blue: 0 }
    }
}
impl FromStr for Run {
    type Err = ();

    /// convert " 3 blue, 4 red"," 1 red, 2 green, 6 blue", "2 green"
    /// to [(Blue,3),(Red,4)], etc
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        #[derive(Debug,Eq, PartialEq,Hash)]
        enum Colour { Red, Green, Blue }

        Ok( input
            .trim()
            .split(',')
            .map(|picked| {
                let mut split = picked.trim().split(' ');
                let count = u32::from_str_radix(split.next().unwrap(), 10).expect("Ops!");
                let colour = match split.next().unwrap().trim() {
                    "red" => Colour::Red,
                    "green" => Colour::Green,
                    "blue" => Colour::Blue,
                    err => {println!("What's this \"{err}\"?!"); unreachable!("Shouldn't be here")}
                };
                (colour,count)
            })
            .fold(Run::default(),|mut run, (col, val)| {
                match col {
                    Colour::Red => run.red = val,
                    Colour::Green => run.green = val,
                    Colour::Blue => run.blue = val
                }
                run
            })
        )
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    runs: Vec<Run>,
    max: Run
}

impl Game {
    fn is_feasible(&self, game: &Game) -> bool {
        let Run{ red,green,blue} = game.runs[0];
        self.runs
            .iter()
            .all(|run| run.red <= red && run.blue <= blue && run.green <= green )
    }
    fn power(&self) -> u32 {
        let Run { red, green, blue} = self.max;
        red * green * blue
    }
}

impl FromStr for Game {
    type Err = ();

    /// should parse the string "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    /// Game { 1, [ {(Blue,3),(Red,4)},{(Red,1),(Green,2),(Blue,6)},{(Green,2)} ]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (mut max_r, mut max_b, mut max_g) = (0,0,0);
        let mut gsplit = input.split(':');
        let id = u32::from_str_radix(gsplit.next().unwrap().split(' ').last().unwrap(), 10).expect("Ops");
        let runs = gsplit
            .next().unwrap()
            .split(';')
            .map(|run| Run::from_str(run).expect("Ops!") )
            .inspect(|run| {
                max_r = max(max_r, run.red);
                max_b = max(max_b, run.blue);
                max_g = max(max_g, run.green);
            })
            .collect::<Vec<_>>();

        Ok(Game {
            id, runs,
            max: Run { red: max_r, green: max_g, blue: max_b },
        })
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
    fn test_game_feasible() {

        let gref = Game {
            id: 0,
            runs: vec![
                Run { red: 12, blue:14, green:13 }
            ],
            max: Run { red: 12, blue:14, green:13 }
        };

        let sum = INPUT.lines()
            .map(|game| Game::from_str(game).expect("Ops!"))
            .filter(|game| game.is_feasible(&gref) )
            .map(|game| game.id)
            .sum::<u32>();

        assert_eq!(8,sum);
    }

    #[test]
    fn test_game_power() {
        let sum = INPUT.lines()
            .map(|game| Game::from_str(game).expect("Ops!"))
            .map(|game| game.power() )
            .inspect(|n| println!("{n}"))
            .sum::<u32>();

        assert_eq!(2286,sum);
    }

    #[test]
    fn test_parse_input() {
        let input = "Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_str(input).expect("Ops!");
        println!("{:?} = {}",game, game.power());
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