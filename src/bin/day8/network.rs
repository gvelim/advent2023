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

pub enum NetworkParseErr {
    UnknownError
}

impl Display for NetworkParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkParseErr::UnknownError => write!(f, "Unkown Network parsing error")
        }
    }
}

impl FromStr for Network {
    type Err = NetworkParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let net = s.lines()
            .map(|line| {
                let mut iter = line
                    .split([' ', '=', '(', ')', ','])
                    .filter(|&s| !s.is_empty());
                (
                    iter.next().unwrap().into(),
                    (
                        iter.next().unwrap().into(),
                        iter.next().unwrap().into()
                    )
                )
            })
            .collect::<HashMap<Rc<str>,(Rc<str>,Rc<str>)>>();

        if !net.is_empty() {
            Ok(Network { net })
        } else {
            Err(NetworkParseErr::UnknownError)
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
