use std::{
    collections::HashMap,
    fmt::Display,
    rc::Rc,
    str::FromStr
};

#[derive(Debug,PartialEq)]
pub(crate) struct Network {
    pub(crate) net: HashMap<Rc<str>,(Rc<str>, Rc<str>)>,
}

impl Network {
    pub(crate) fn iter(
        self: Rc<Self>,
        start: &str,
        turns: impl Iterator<Item=char>
    ) -> NetworkIter<impl Iterator<Item=char>>
    {
        NetworkIter { net: self, key: start.into(), turns }
    }
}

#[derive(Debug,PartialEq)]
pub enum NetworkParseErr {
    UnknownError(String)
}

impl Display for NetworkParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkParseErr::UnknownError(d) => write!(f, "{}",d)
        }
    }
}

impl FromStr for Network {
    type Err = NetworkParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut errors = vec![];
        let net = s
            .lines()
            .enumerate()
            .map(|(n, line)| {
                let mut iter = line
                    .split([' ', '=', '(', ')', ','])
                    .filter(|&s| !s.is_empty());
                (
                    n,
                    iter.next().unwrap_or("") as &str,
                    (
                        iter.next().unwrap_or("") as &str,
                        iter.next().unwrap_or("") as &str
                    )
                )
            })
            .map(|(line, k,(l,r))|{
                match (k.len(), l.len(), r.len()) {
                    (0,0,0)|(0,_,_)|(_,0,_)|(_,_,0) => errors.push(line+1),
                    _ => ()
                };
                (k.into(),(l.into(),r.into()))
            })
            .collect::<HashMap<Rc<str>,(Rc<str>,Rc<str>)>>();

        if errors.is_empty() {
            Ok(Network { net })
        } else {
            Err(NetworkParseErr::UnknownError(format!("Error(s) found in line(s) {:?}",errors)))
        }
    }
}

pub(crate) struct NetworkIter<I> where I: Iterator<Item=char> {
    net: Rc<Network>,
    key: Rc<str>,
    turns: I
}

impl<I> Iterator for NetworkIter<I> where I: Iterator<Item=char> {
    type Item = Rc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.turns.next() {
            Some('L') => self.net.net.get(&self.key).map(|(l,_)| l.clone()),
            Some('R') => self.net.net.get(&self.key).map(|(_,r)| r.clone()),
            _ => unreachable!()
        }
        .inspect(|next| self.key = next.clone() )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_parse_nodes() {
        let input: &str = "AAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        let Ok(net) = input.parse::<Network>() else { panic!("test_parse_nodes::failed to parse") };

        println!("{:?}",net);
        assert_eq!(
            Network { net: HashMap::from([
                ("ZZZ".into(), ("ZZZ".into(), "ZZZ".into())),
                ("AAA".into(), ("BBB".into(), "BBB".into())),
                ("BBB".into(), ("AAA".into(), "ZZZ".into()))
            ])},
            net
        )
    }

    #[test]
    fn test_parse_error() {
        let input: &str = "\
            AAA = (BBB,BBB)\n\
            AAA= (, BBB)\n\
            BBB =(AAA, ZZZ)\n\
            = (AAA, ZZZ)\n\
            ZZZ=(ZZZ,ZZZ)\n\
            ZZZ = (ZZZ,)\n\
            =(,)\n\
            AAA = ZZZ, BBB\n\
            AAA ZZZ, (BBB) XXX";
        let parse = input.parse::<Network>();
        match parse {
            Ok(_) => panic!("test_parse_error::parsing shall result in error"),
            Err(e) => {
                println!("{}",e);
                assert_eq!(
                    e,
                    NetworkParseErr::UnknownError("Error(s) found in line(s) [2, 4, 6, 7]".to_string())
                )
            },
        }
    }
}
