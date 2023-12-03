fn main() {

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
        .map(|line| line.split([':',';']).collect::<Vec<_>>() )
        .map(|s| {(
            Game {
                id: s[0].chars().last().unwrap().to_digit(10).expect("Ops!"),
                red: 0, green: 0, blue: 0
            }, s
        )})
        .fold( Vec::<Game>::new(), |mut arr, (mut game, runs)| {
            runs.iter()
                .skip(1)
                .map(|run| {
                    run.split(',')
                        .map(|ball| {
                            let mut s = ball.trim().split(' ');
                            let val = u32::from_str(s.next().unwrap()).unwrap_or_else(|e| panic!("{e}"));
                            match s.next().unwrap().trim() {
                                "green" => game.green += val,
                                "blue" => game.blue += val,
                                "red" => game.red += val,
                                d => println!("{d}")
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

        arr.iter().all(|g| {println!("{:?} = {}",g, g.sum());true} );

        assert!(true);

    }

}