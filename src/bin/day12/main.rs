fn main() {

}

#[cfg(test)]
mod test {
    use std::iter::repeat;


    #[derive(Debug)]
    struct Combinations {
        out: Vec<String>
    }
    impl Combinations {
        fn parse(&mut self, inp: &str, count: &[usize]) -> Option<String> {
            let mut string = String::new();
            let mut iter = inp.chars();
            let mut pat = String::new();

            if inp.is_empty() && count.is_empty() {
                return Some(String::new());
            } else {
                 pat.extend(repeat('#').take(count[0]));
            }

            println!("{:?}",(&inp,count));

            loop {
                match iter.next() {
                    Some('?') => {
                        self.parse(&format!("{}{}{}",string,'#',iter.as_str()), count);
                        self.parse(&format!("{}{}{}",string,'.',iter.as_str()), count);
                    }
                    Some('.') | None if !string.is_empty() => {
                        if string.ends_with( &pat ) {
                            return self
                                .parse(iter.as_str(), &count[1..])
                                .map(|a| { 
                                    println!("\tFound! {:?}",(pat, count[0], &a, &string));
                                    a
                                })
                        } else { 
                            println!("\tMissed! {:?}",string);
                            return None 
                        }
                    },
                    Some(c) => string.push(c),
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

        let c = comb.parse(&"??.###", &[1,3]);
        println!("{:?}",(c,&comb));
    }
}