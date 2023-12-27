#![feature(iter_collect_into)]

fn main() {

}

#[cfg(test)]
mod test {
    
    #[derive(Debug)]
    struct Combinations {
        out: Vec<String>
    }
    impl Combinations {
        fn parse(&mut self, inp: &str, count: &[usize]) -> Option<Vec<String>> {
            let mut buf = String::new();
            let mut iter = inp.chars();
            let mut out = vec![];

            println!("{:?}",(&inp,&count, inp.len(), (&count.iter().sum::<usize>() + &count.iter().count())));
            if inp.is_empty() && count.is_empty() {
                println!("reached bottom!!");
                return Some(vec![]);
            }

            loop {
                match iter.next() {
                    Some('?') => {
                        // print!("\tFork in # ->");
                        self
                            .parse(&format!("{}#{}",buf,iter.as_str()), count)
                            .inspect(|v| println!("\tFork out # {:?}",v))
                            .map(|v|
                                v.into_iter().collect_into(&mut out)
                            );
                        // print!("\tFork in .. ->");
                        self
                            .parse(&format!("{}.{}",buf,iter.as_str()), count)
                            .inspect(|v| println!("\tFork out .. {:?}",v))
                            .map(|v|
                                v.into_iter().collect_into(&mut out)
                            );

                        return if out.is_empty() { None } else { Some(out) }
                    },
                    Some('.') | None if !buf.is_empty() => {
                        if buf.trim_matches('.').len() == count[0] {
                            println!("\t->{}",buf);
                            return self
                                .parse(iter.as_str(), &count[1..])
                                .inspect(|v| println!("\tRet:{:?}",v))
                                .map(|v| {
                                    if buf.len() < inp.len() { buf.push('.') };
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
                        let hashes = buf.chars().filter(|c| '#'.eq(c)).count();
                        println!("{hashes}::{:?}",buf);
                        if hashes > count[0] {
                            println!("abort");
                            return None
                        }
                        buf.push(c)
                    },
                    None => return None
                }                
            }
        }
    }

    #[test]
    fn test_parse() {

        let mut comb = Combinations {
            out: vec![],
        };

        let c = comb.parse(&"????.##", &[1,1,2]);
        println!("{:?}",(c,comb.out));
    }
}