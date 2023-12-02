#![feature(result_option_inspect)]

use std::ops::Not;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day1/input.txt").unwrap_or_else(|e| panic!("{e}"));

    let sum = inp.lines()
        .filter_map(|line| extract_first_last(line))
        .sum::<u32>();
    println!("Sum = {sum}");
}

fn extract_first_last(inp: &str) -> Option<u32> {
    let mut tmp = vec![];
    inp.chars()
        .filter(|c| c.is_digit(10))
        .for_each(|c| tmp.push(c) );
    tmp.is_empty()
        .not()
        .then(|| {
            10 * tmp.first().unwrap().to_digit(10).expect("Failed to convert")
                + tmp.last().unwrap().to_digit(10).expect("Failed to convert")
        })
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: [&str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f","treb7uchet"];
    #[test]
    fn test_extract_digit() {
        INPUT.iter()
            .for_each(|str|{
                print!("{str:?} : ");
                println!(" -> {:?}",extract_first_last(str))
            })
    }
}