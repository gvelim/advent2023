use std::collections::HashMap;

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

static HMDIGITS: HashMap<&str,u32> = HashMap::from([
    ("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)
]);


fn main() {
    let inp = std::fs::read_to_string("src/bin/day1/input.txt").unwrap_or_else(|e| panic!("{e}"));

    let sum = inp.lines()
        .filter_map(|line| extract_first_last(line))
        .sum::<u32>();
    println!("Sum = {sum}");
}

fn extract_first_last(inp: &str) -> Option<u32> {
    use std::ops::Not;

    let mut tmp = vec![];

    inp.chars()
        .filter(|c| c.is_digit(10))
        .for_each(|c| tmp.push(c.to_digit(10).expect("Failed to convert")) );

    tmp.is_empty()
        .not()
        .then(|| 10 * tmp.first().unwrap() + tmp.last().unwrap() )
}

fn extract_first_last_part2(input: &str) -> Option<u32> {
    use std::ops::Not;

    // String Buffer to store non-numeric chars for lateral processing
    let mut buf = String::new();

    // for every char in the input string
    let tmp = input
        .chars()
        .filter_map(|c| {
            match c {
                // if digit convert to numeric
                '1'..='9' => c.to_digit(10),
                // if non-digit
                'a'..='z' => {
                    // push into the string
                    buf.push(c);
                    // For every DIGIT name
                    DIGITS
                        .iter()
                        // check if the string we have in BUF matches any of the DIGIT names
                        .filter_map(|d| {
                            // if BUF doesn't match any return NONE
                            if !buf.ends_with(d) { None }
                            else {
                                // we have a match
                                println!("BUF:{buf}");
                                // clear the buffer
                                buf.clear();
                                match d {
                                    &"one" => Some(1),
                                    &"two" => Some(2),
                                    &"three" => Some(3),
                                    &"four" => Some(4),
                                    &"five" => Some(5),
                                    &"six" => Some(6),
                                    &"seven" => Some(7),
                                    &"eight" => Some(8),
                                    &"nine" => Some(9),
                                    _ => None
                                }
                            }
                        })
                        .next()
                },
                _ => None
            }
        })
        .collect::<Vec<_>>();

    tmp.is_empty()
        .not()
        .then(|| 10 * tmp.first().unwrap() + tmp.last().unwrap() )
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: [&str; 7] = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen"
    ];

    #[test]
    fn test_extract_digit() {
        INPUT.iter()
            .for_each(|str|{
                print!("{str:?} : ");
                println!(" -> {:?}",extract_first_last(str))
            })
    }

    #[test]
    fn test_string_in_string() {

        INPUT.iter()
            .for_each(|input| {
                print!("Inp: {input}");
                println!("Found: {:?}", extract_first_last_part2(input) );
            });
        assert!(true)
    }
}