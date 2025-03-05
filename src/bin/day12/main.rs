mod combinator;

use rayon::prelude::*;
use combinator::*;

fn main() {
    let run_part = |arr: &Vec<(String,Vec<usize>)>| -> usize {
        arr.par_iter()
            .map(|(broken, record)| {
                Combinator::default().get_combinations(broken, record)
            })
            .sum::<usize>()
    };
    let input = std::fs::read_to_string("src/bin/day12/input.txt").expect("Ops");

    let arr = parse(input.as_str(),1);
    let t = std::time::Instant::now();
    println!("Part 1 Sum {:?} - {:?}", run_part(&arr), t.elapsed());

    let arr = parse(input.as_str(),5);
    let t = std::time::Instant::now();
    println!("Part 2 Sum {:?} - {:?}", run_part(&arr), t.elapsed());
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
                .unwrap_or_default()
                .repeat(repetitions);

            ( broken_rec, rec )
        })
        .collect::<Vec<_>>()
}
