

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

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
            .for_each(|inp|{
                let mut i = 0;

                let itr = inp.chars();

                println!("INP: {inp}");

                while i < inp.len()-1 {
                    DIGITS.iter()
                        .any(|digit| {
                            if !inp[i..].starts_with(digit) { false }
                            else {
                                println!("{i}:{}",&inp[i..i+digit.len()],);
                                i += digit.len()-1;
                                true
                            }
                        });
                    i += 1;
                }
            });


        assert!(true)
    }
}