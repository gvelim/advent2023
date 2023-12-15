use std::str::FromStr;

#[derive(Debug,Default)]
pub struct Run {
    pub(crate) red: u32, pub(crate) green: u32, pub(crate) blue: u32
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
                let mut split = picked.trim().split_ascii_whitespace();
                let count = split.next().unwrap().parse::<u32>().expect("Ops!");
                let colour = match split.next().unwrap() {
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
