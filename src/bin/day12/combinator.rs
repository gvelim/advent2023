use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Combinator<'a> {
    mem: RefCell<HashMap<(String, &'a [usize]),Option<usize>>>
}

impl<'a> Combinator<'a> {
    pub(crate) fn get_combinations(&self, inp: &str, count: &'a [usize]) -> Option<usize> {
        let mut iter = inp.chars();
        let key = (iter.as_str().to_string(), count);

        if let Some(&val) = self.mem.borrow().get(&key) {
            // println!("Cached: {:?}", (&key, val));
            return val
        }

        // println!("{:?}", (&inp, &count, inp.len(), &count.iter().sum::<usize>()));
        match (inp.is_empty(), count.is_empty()) {
            (true, true) => {
                // println!("Matching combination!! no trailing `...`");
                return Some(1);
            },
            (false, true) if !inp.contains('#') => {
                // println!("Matching combination!! trailing '.??.' ");
                return Some(1);
            },
            (false, true) => {
                // println!("Abort - ran out of counts with # still remain");
                return None;
            }
            (true, false) => {
                // println!("Abort - ran out of string");
                return None
            },
            (_, _) => if inp.len() < count.iter().sum::<usize>() { return None }
        }

        let mut hashes = 0;
        let mut buf = String::new();

        loop {
            match iter.next() {
                Some('?') => {
                    let ret =
                        self.get_combinations(&format!("{}#{}", buf, iter.as_str()), count).unwrap_or(0) +
                            self.get_combinations(&format!("{}.{}", buf, iter.as_str()), count).unwrap_or(0);
                    return if ret == 0 { None } else { Some(ret) }
                },
                Some('.') | None if hashes > 0 => {
                    if buf.len() < inp.len() { buf.push('.') };
                    return if hashes == count[0] {
                        let ret= self.get_combinations(iter.as_str(), &count[1..]);
                        self.mem.borrow_mut().entry(key).or_insert(ret);
                        ret
                    } else {
                        None
                    }
                },
                Some(c) => {
                    buf.push(c);
                    hashes += if '#' == c { 1 } else { 0 };
                    if hashes > count[0] {
                        return None
                    }
                },
                None => return None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse;

    #[test]
    fn test_parse_combinations() {
        let input = std::fs::read_to_string("src/bin/day12/sample.txt").expect("Ops");

        let arr = parse(input.as_str(), 5);

        let sum = arr.iter()
            .inspect(|(a,b)| print!("\"{a}\" <=> {:?}",b))
            .map(|(broken, record)| {
                Combinator::default().get_combinations(&broken, &record)
            } )
            .map(|comb| {
                comb.unwrap_or(0)
            } )
            .inspect(|combo| println!(" = {:?}",combo))
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