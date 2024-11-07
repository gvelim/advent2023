use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub(crate) struct Network<'a> {
    pub(crate) net: HashMap<&'a str,(&'a str, &'a str)>,
}

impl Network<'_> {

    pub(crate) fn iter<'a>(
        &'a self,
        start: &'a str,
        turns: impl Iterator<Item=char>
    ) -> NetworkIter<'a, impl Iterator<Item=char>>
    {
        NetworkIter { net: self, key: start, turns }
    }

    pub(crate) fn parse(s: &str) -> Network<'_> {
        Network {
            net: s.lines()
                .map(|line| {
                    let mut iter = line.split([' ', '=', '(', ')', ','])
                        .filter(|&s| !s.is_empty());
                    (iter.next().unwrap(), (iter.next().unwrap(),iter.next().unwrap()))
                })
                .collect::<HashMap<&str,(&str,&str)>>()
        }
    }
}

pub(crate) struct NetworkIter<'a,I> where I: Iterator<Item=char> {
    net: &'a Network<'a>,
    key: &'a str,
    turns: I
}

impl<'a, I> Iterator for NetworkIter<'a, I> where I: Iterator<Item=char> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.turns.next() {
            Some('L') => self.net.net.get(self.key).map(|(l,_)| *l),
            Some('R') => self.net.net.get(self.key).map(|(_,r)| *r),
            _ => unreachable!()
        }
        .inspect(|&next| self.key = next )
    }
}
