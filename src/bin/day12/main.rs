#![feature(iter_collect_into)]

use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").expect("Ops");

    let arr = parse(input.as_str(),5);

    let t = std::time::Instant::now();
    let sum = arr.into_par_iter()
        // .inspect(|(a,b)| println!("\"{a}\" <=> {:?}",b))
        .map(|(broken, record)| ((broken.clone(),record.clone()), get_combinations(&broken, &record)) )
        // .inspect(|d| println!("{:?}",d))
        .map(|(b,comb)| {
            (b, comb.unwrap()
                .into_iter()
                // .inspect(|combo| println!("{:?}",combo))
                .count())
        } )
        .map(|(b,d)|{ println!("{:?} = {:?} - {:?}",b,d, t.elapsed()); d })
        .sum::<usize>();

    println!("Sum {:?} - {:?}",sum, t.elapsed());
}

fn get_combinations(inp: &str, count: &[usize]) -> Option<Vec<String>> {
    let mut buf = String::new();
    let mut iter = inp.chars();
    let mut out = vec![];

    // println!("{:?}", (&inp, &count, inp.len(), &count.iter().sum::<usize>()));
    match (inp.is_empty(), count.is_empty()) {
        (true, true) => {
            // println!("Matching combination!! no trailing `...`");
            return Some(vec![]);
        },
        (false, true) if !inp.contains('#') => {
            // println!("Matching combination!! trailing '.??.' ");
            return Some(vec![
                (0..inp.len()).map(|_| '.').collect()
            ]);
        },
        (false, true) => {
            // println!("Abort - ran out of counts with # still remain");
            return None;
        }
        (true, false) => {
            // println!("Abort - ran out of string");
            return None;
        },
        (_, _) => if inp.len() < count.iter().sum::<usize>() {
            // println!("Abort - Less than total count");
            return None;
        }
    }

    loop {
        match iter.next() {
            Some('?') => {
                // print!("\tFork in # -> {:?}", format!("{}#{}", buf, iter.as_str()));
                get_combinations(&format!("{}#{}", buf, iter.as_str()), count)
                    // .inspect(|v| println!("\tFork out # {:?}", v))
                    .map(|v|
                        v.into_iter().collect_into(&mut out)
                    );
                // print!("\tFork in .. ->");
                get_combinations(&format!("{}.{}", buf, iter.as_str()), count)
                    // .inspect(|v| println!("\tFork out .. {:?}", v))
                    .map(|v|
                        v.into_iter().collect_into(&mut out)
                    );

                return if out.is_empty() { None } else { Some(out) }
            },
            Some('.') | None if buf.contains('#') => {
                if buf.len() < inp.len() { buf.push('.') };

                if buf.trim_matches('.').len() == count[0]
                {
                    // println!("\t->{}", buf);
                    return get_combinations(iter.as_str(), &count[1..])
                        // .inspect(|v| println!("\tRet:{:?}", v))
                        .map(|v| {
                            if !v.is_empty() {
                                v.into_iter().map(|s| buf.clone() + &s ).collect_into(&mut out);
                            } else {
                                out.push(buf)
                            }
                            out
                        })
                } else {
                    // println!("\t Missed!");
                    return None
                }
            },
            Some(c) => {
                buf.push(c);
                let hashes = buf.chars().filter(|c| '#'.eq(c)).count();
                // println!("{hashes}::{:?}", buf);
                if hashes > count[0] {
                    // println!("abort");
                    return None
                }
            },
            None => return None
        }
    }
}

fn parse(input:&str, repetitions: usize) -> Vec<(String, Vec<usize>)> {
    input.lines()
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let mut broken_rec = split.next()
                .map(|s|{ let mut t = String::from(s); if repetitions > 1 { t.push('?') } t })
                .unwrap()
                .repeat(repetitions);
            if repetitions > 1 { broken_rec.pop(); }
            let rec = split.next()
                .map(|s|{
                    s.split(',')
                        .map(|n| n.parse::<usize>().expect("Ops!")).collect::<Vec<_>>()
                })
                .unwrap_or(vec![])
                .repeat(repetitions);

            ( broken_rec, rec )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{get_combinations, parse};

    #[test]
    fn test_parse_combinations() {
        let input = std::fs::read_to_string("src/bin/day12/sample.txt").expect("Ops");

        let arr = parse(input.as_str(), 5);

        let sum = arr.iter()
            .inspect(|(a,b)| print!("\"{a}\" <=> {:?}",b))
            .map(|(broken, record)| get_combinations(&broken, &record) )
            .map(|comb| {
                comb.unwrap()
                    .into_iter()
                    .count()
            } )
            .inspect(|combo| println!(" = {:?}",combo))
            .sum::<usize>();

        println!("Sum {:?}",sum);
    }
    #[test]
    fn test_combinations() {
        let (inp, counts) = ("?###??????????###??????????###??????????###??????????###????????", [3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1]);

        println!("{:?}", get_combinations(inp,&counts))
    }
}