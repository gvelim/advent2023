fn main() {
    let inp = std::fs::read_to_string("src/bin/day1/input.txt").unwrap_or_else(|e| panic!("{e}"));

    let sum = inp.lines()
        .filter_map(|line| extract_first_last_part1(line))
        .sum::<u32>();
    println!("Part 1 -> Sum = {sum}");

    let sum = inp.lines()
        .filter_map(|line| extract_first_last_part2(line))
        .inspect(|num| println!("Result: {num}"))
        .sum::<u32>();
    println!("Part 2 -> Sum = {sum}");
}

fn extract_first_last_part1(inp: &str) -> Option<u32> {
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
    static DIGITS: [(&str,u32); 9] = [
        ("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)
    ];

    // String Buffer to store non-numeric chars for lateral processing
    let mut buf = String::new();
    print!("Inp: {input} -> ");

    // for every char in the input string
    let mut parser = input
        .chars()
        .filter_map(|c| {
            match c {
                // if digit convert to numeric
                '1'..='9' => {
                    buf.clear();
                    c.to_digit(10)
                },
                // if non-digit
                'a'..='z' => {
                    // push into the string
                    buf.push(c);
                    // For every DIGIT name ... return Some(num) or None
                    DIGITS
                        .iter()
                        // check if the string we have in BUF matches any of the DIGIT names
                        .filter_map(|(d,numeric)| {
                            // if BUF doesn't match any return NONE
                            if !buf.ends_with(d) { None }
                            else {
                                // we have a match
                                print!("{buf},");
                                // clear the buffer
                                buf.clear();
                                Some(*numeric)
                            }
                        })
                        .next()
                },
                _ => None
            }
        });

    let ret = parser.next().unwrap();
    Some(ret * 10 + parser.last().unwrap_or(ret))
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
    fn test_extract_digit_numeric() {
        let sum = INPUT.iter()
            .filter_map(|str|{
                print!("{str:?} : ");
                let val = extract_first_last_part1(str);
                println!(" -> {:?}",val );
                val
            })
            .sum::<u32>();
        assert_eq!(sum, 209)
    }

    #[test]
    fn test_part2() {

        let sum = INPUT.iter()
            .filter_map(|input| {
                let val = extract_first_last_part2(input);
                println!("Found: {:?}", val);
                val
            })
            .sum::<u32>();
        assert_eq!(sum, 281)
    }
}