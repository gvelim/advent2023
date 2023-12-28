#![feature(iter_collect_into)]

fn main() {

}

fn get_combinations(inp: &str, count: &[usize]) -> Option<Vec<String>> {
    let mut buf = String::new();
    let mut iter = inp.chars();
    let mut out = vec![];

    println!("{:?}", (&inp, &count, inp.len(), &count.iter().sum::<usize>()));
    match (inp.is_empty(), count.is_empty()) {
        (true, true) => {
            println!("Matching combibation!!");
            return Some(vec![]);
        },
        (false, true) => {
            println!("Abort - ran out of counts");
            return None;
        },
        (true, false) => {
            println!("Abort - ran out of string");
            return None;
        },
        (false, false) => if inp.len() < count.iter().sum::<usize>() {
            println!("Abort - Less than total count");
            return None;
        }
    }

    loop {
        match iter.next() {
            Some('?') => {
                print!("\tFork in # -> {:?}", format!("{}#{}", buf, iter.as_str()));
                get_combinations(&format!("{}#{}", buf, iter.as_str()), count)
                    .inspect(|v| println!("\tFork out # {:?}", v))
                    .map(|v|
                        v.into_iter().collect_into(&mut out)
                    );
                print!("\tFork in .. ->");
                get_combinations(&format!("{}.{}", buf, iter.as_str()), count)
                    .inspect(|v| println!("\tFork out .. {:?}", v))
                    .map(|v|
                        v.into_iter().collect_into(&mut out)
                    );

                return if out.is_empty() { None } else { Some(out) }
            },
            Some('.') | None if buf.contains('#') => {
                if buf.len() < inp.len() { buf.push('.') };

                if buf.chars().filter(|c| '#'.eq(c)).count() == count[0]
                {
                    println!("\t->{}", buf);
                    return get_combinations(iter.as_str(), &count[1..])
                        .inspect(|v| println!("\tRet:{:?}", v))
                        .map(|v| {
                            if !v.is_empty() {
                                v.into_iter().for_each(|s| {
                                    out.push(buf.clone() + &s)
                                })
                            } else {
                                out.push(buf)
                            }
                            out
                        })
                } else {
                    println!("\t Missed!");
                    return None
                }
            },
            Some(c) => {
                buf.push(c);
                let hashes = buf.chars().filter(|c| '#'.eq(c)).count();
                println!("{hashes}::{:?}", buf);
                if hashes > count[0] {
                    println!("abort");
                    return None
                }
            },
            None => return None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::get_combinations;

    #[test]
    fn test_parse_combinations() {

        let c = get_combinations(&"??", &[1]);
        println!("{:?}",c);
    }
}