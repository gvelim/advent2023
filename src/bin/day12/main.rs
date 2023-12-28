#![feature(iter_collect_into)]

use std::cell::RefCell;
use std::collections::HashMap;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn main() {
    let input = std::fs::read_to_string("src/bin/day12/input.txt").expect("Ops");

    let arr = parse(input.as_str(),3);

    let t = std::time::Instant::now();
    let sum = arr.par_iter()
        // .inspect(|(a,b)| println!("\"{a}\" <=> {:?}",b))
        .map(|(broken, record)| {
            ((broken.clone(),record.clone()), Combinator::default().get_combinations(&broken, &record))
        } )
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

#[derive(Default)]
struct Combinator<'a> {
    mem: RefCell<HashMap<(String, &'a [usize]),Option<Vec<String>>>>
}

impl<'a> Combinator<'a> {
    fn get_combinations(&self, inp: &'_ str, count: &'a [usize]) -> Option<Vec<String>> {
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

        let key = (iter.as_str().to_string(), count);
        let mut hashes = 0;
        loop {
            match iter.next() {
                Some('?') => {
                    // print!("\tFork in # -> {:?}", format!("{}#{}", buf, iter.as_str()));
                    self.get_combinations(&format!("{}#{}", buf, iter.as_str()), count)
                        // .inspect(|v| println!("\tFork out # {:?}", v))
                        .map(|v|
                            v.into_iter().collect_into(&mut out)
                        );
                    // print!("\tFork in .. ->");
                    self.get_combinations(&format!("{}.{}", buf, iter.as_str()), count)
                        // .inspect(|v| println!("\tFork out .. {:?}", v))
                        .map(|v|
                            v.into_iter().collect_into(&mut out)
                        );

                    return if out.is_empty() { None } else { Some(out) }
                },
                Some('.') | None if hashes > 0 => {
                    if buf.len() < inp.len() { buf.push('.') };

                    if hashes == count[0]
                    {
                        if let Some(val) = self.mem.borrow().get(&key) {
                            // println!("Cached: {:?}", (&key, val));
                            return val.clone()
                        }
                        // println!("\t->{}", buf);
                        return self.get_combinations(iter.as_str(), &count[1..])
                            // .inspect(|v| println!("\tRet:{:?}", v))
                            .map(|v| {
                                if !v.is_empty() {
                                    v.into_iter().map(|s| buf.clone() + &s).collect_into(&mut out);
                                } else {
                                    out.push(buf)
                                }
                                out
                            })
                            .map(|vec| {
                                if count.len() > 2 {
                                    self.mem.borrow_mut().entry(key.clone()).or_insert(Some(vec.clone()));
                                }
                                println!("Hash Key{:?} -> {:?}",&key,&self.mem.borrow().get(&key));
                                vec
                            })
                    } else {
                        // println!("\t Missed!");
                        return None
                    }
                },
                Some(c) => {
                    buf.push(c);
                    hashes += if '#' == c { 1 } else { 0 };
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
    use super::*;

    #[test]
    fn test_parse_combinations() {
        let input = std::fs::read_to_string("src/bin/day12/sample.txt").expect("Ops");

        let arr = parse(input.as_str(), 3);

        let sum = arr.iter()
            // .inspect(|(a,b)| print!("\"{a}\" <=> {:?}",b))
            .map(|(broken, record)| {
                Combinator::default().get_combinations(&broken, &record)
            } )
            .map(|comb| {
                comb.unwrap()
                    .into_iter()
                    .count()
            } )
            // .inspect(|combo| println!(" = {:?}",combo))
            .sum::<usize>();

        println!("Sum {:?}",sum);
    }
    #[test]
    fn test_combinations() {
        let (inp, counts) =  ("?###??????????###??????????###????????",[3, 2, 1, 3, 2, 1, 3, 2, 1, ]);
        let c = Combinator::default();

        println!("{:?}", c.get_combinations(inp,&counts))
    }
}