use std::time::Instant;

fn main() {
    let inp = std::fs::read_to_string("src/bin/day1/input.txt").unwrap_or_else(|e| panic!("{e}"));

    let t = Instant::now();
    println!("Part 1 -> Sum = {:?} - {:?}", sum_up(&inp, ParserDigits), t.elapsed());

    let t = Instant::now();
    println!("Part 2 -> Sum = {:?} - {:?}", sum_up(&inp, ParserNumerics), t.elapsed());
}

fn sum_up(inp:&str, p: impl Parse) -> u32 {
    inp.lines()
        .filter_map(|line| {
            let mut iter = p.parser(line);
            iter.next().map(|f| 10*f + iter.last().unwrap_or(f) )
        })
        .sum::<u32>()
}

trait Parse {
    fn parser<'a>(&self, inp: &'a str) -> impl Iterator<Item = u32> + 'a;
}

struct ParserDigits;
impl Parse for ParserDigits {
    fn parser<'a>(&self, inp: &'a str) -> impl Iterator<Item = u32> + 'a {
        inp.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| (c as u8 - b'0') as u32)
    }
}

struct ParserNumerics;
impl Parse for ParserNumerics {
    fn parser<'a>(&self, input: &'a str) -> impl Iterator<Item = u32> + 'a {
        static DIGITS: [(&str,u32); 9] = [
            ("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)
        ];

        let mut buf = String::with_capacity(60);
        input.chars()
            .filter_map(move |c| {
                match c {
                    '0'..='9' => Some((c as u8 - b'0') as u32),
                    'a'..='z' => {
                        buf.push(c);
                        DIGITS.iter()
                            .filter_map(|(d, numeric)|
                                if !buf.ends_with(d) { None } else { Some(*numeric) }
                            )
                            .next()
                    },
                    _ => None
                }
            })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str =
        "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(sum_up(INPUT, ParserDigits), 209)
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_up(INPUT, ParserNumerics), 281)
    }
}
