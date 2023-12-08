use crate::run::Run;
use crate::game::Game;
use std::time::Instant;

mod game;
mod run;

fn main() {
    let input = std::fs::read_to_string("src/bin/day2/input.txt").unwrap_or_else(|e| panic!("{e}"));
    let rref = Run { red: 12, blue:14, green:13 };

    let games = input
        .lines()
        .map(|game| game.parse::<Game>().expect("Ops!"))
        .collect::<Vec<_>>();

    let t = Instant::now();
    let sum = games.iter()
        .filter(|game| game.is_feasible(&rref))
        .map(|game| game.id )
        .sum::<u32>();

    println!("Part 1 : Sum = {sum} - {:?}", t.elapsed());

    let t = Instant::now();
    let sum = games.iter()
        .map(|game| game.power() )
        .sum::<u32>();

    println!("Part 2 : Sum = {sum} - {:?}", t.elapsed());
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

        let gref = Run { red: 12, blue:14, green:13 };

        let sum = INPUT.lines()
            .map(|game| game.parse::<Game>().expect("Ops!"))
            .filter(|game| game.is_feasible(&gref) )
            .map(|game| game.id)
            .sum::<u32>();

        assert_eq!(8,sum);
    }

    #[test]
    fn test_game_power() {
        let sum = INPUT.lines()
            .map(|game| game.parse::<Game>().expect("Ops!"))
            .map(|game| game.power() )
            .inspect(|n| println!("{n}"))
            .sum::<u32>();

        assert_eq!(2286,sum);
    }

    #[test]
    fn test_parse_input() {
        let input = "Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = input.parse::<Game>().expect("Ops!");
        println!("{:?} = {}",game, game.power());
        assert!(true);
    }

    #[test]
    fn test_run_parse() {
        let input = [" 3 blue, 4 red"," 1 red, 2 green, 6 blue"," 2 green"];
        input.iter()
            .for_each(|inp| {
                let run = inp.parse::<Run>().expect("Ops!");
                println!("Run: {:?}",run);
            });
    }
}