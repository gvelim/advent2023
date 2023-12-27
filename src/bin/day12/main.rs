fn main() {

}

#[cfg(test)]
mod test {

    #[derive(Debug)]
    struct Combinations {
        out: Vec<String>
    }
    impl Combinations {
        fn parse(&mut self, inp: &str, count: &[usize]) -> Option<String> {
            let mut string = String::new();
            let mut iter = inp.chars();

            if inp.is_empty() && count.is_empty() {
                return Some(String::new());
            }

            println!("{:?}",(&inp,count));

            loop {
                match iter.next() {
                    Some('?') => {
                        println!("#{:?}",self.parse(&format!("{}{}{}",string,'#',iter.as_str()), count));
                        println!(".{:?}",self.parse(&format!("{}{}{}",string,'.',iter.as_str()), count));
                    }
                    Some('.') | None if !string.is_empty() => {
                        if string.trim_matches('.').len() == count[0] {
                            println!("\tGot! {:?}",(count[0], &string));
                            return self
                                .parse(iter.as_str(), &count[1..])
                                .map(|a| {
                                    println!("\tFound! {:?}",(count[0], &a, &string));
                                    string.push_str(&a);
                                    string
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

        let c = comb.parse(&"??.##", &[1,2]);
        println!("{:?}",(c,comb.out));
    }
}