use std::cmp::max;
use std::os::macos::raw::stat;

fn main() {
    let input = std::fs::read_to_string("src/bin/day2/input.txt").unwrap_or_else(|e| panic!("{e}"));
    let arr = parse_input(input.as_str());

    let g_ref = Game {id: 0, red: 12, green: 13, blue: 14};

    let num = arr.iter()
        .filter(|g| {
            g.red < g_ref.red
                && g.green <= g_ref.green
                && g.blue <= g_ref.blue
        })
        .map(|g| g.id)
        .sum::<u32>();

    arr.iter().for_each(|g| println!("{:?} = {}",g, g.sum()) );
    println!("Part 1 : Probable games = {num}");
}

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32
}

impl Game {
    fn sum(&self) -> u32 {
        self.red + self.blue + self.green
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    use std::str::FromStr;

    input.lines()
        .map(|line| line.split([':',';']) )
        .map(|mut s| {(
            Game {
                id: u32::from_str_radix(
                    s.next()
                            .unwrap()
                            .split(' ')
                            .last()
                            .unwrap(),
                10).expect("Ops!"),
                red: 0, green: 0, blue: 0
            }, s.collect::<Vec<_>>()
        )})
        .fold( Vec::<Game>::new(), |mut arr, (mut game, runs)| {
            runs.iter()
                //.skip(1)
                .map(|run| {
                    run.split(',')
                        .map(|balls| {
                            let mut s = balls.trim().split(' ');
                            let val = u32::from_str(s.next().unwrap()).unwrap_or_else(|e| panic!("{e}"));
                            match s.next().unwrap() {
                                "green" => game.green = max( game.green, val),
                                "blue" => game.blue = max( game.blue, val),
                                "red" => game.red = max( game.red, val),
                                _ => panic!("This shouldn't have happened")
                            }
                        })
                        .all(|_| true)
                })
                .all(|t| t);

            arr.push(game);
            arr
        })
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
        let arr = parse_input(INPUT);
        arr.iter().for_each(|g| println!("{:?} = {}",g, g.sum()) );
        assert!(true);
    }

    #[test]
    fn test_game_compare() {
        let g_ref = Game {id: 0, red: 12, green: 13, blue: 14};
        let arr = parse_input(INPUT);

        let num = arr.iter()
            .filter(|g| {
                g.red < g_ref.red
                    && g.green < g_ref.green
                    && g.blue < g_ref.blue
            })
            .map(|g| g.id)
            .sum::<u32>();
        assert_eq!(num, 8)
    }

}