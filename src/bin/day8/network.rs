use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub(crate) struct Network<'a> {
    pub(crate) net: HashMap<&'a str,(&'a str, &'a str)>,
}

impl Network<'_> {

    pub(crate) fn iter<'a>(&'a mut self, start: &'a str, turns: impl Iterator<Item=char>) -> NetworkIter<'a, impl Iterator<Item=char>> {
        NetworkIter { net: &self.net, start, turns }
    }

    pub(crate) fn par_iter<'a>(&'a mut self, start: &'a [&'a str], turns: impl Iterator<Item=char>) -> ParNetworkIter<'a, impl Iterator<Item=char>> {
        ParNetworkIter { net: &self.net, turns,
            start: start.to_vec()
        }
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
    net: &'a HashMap<&'a str,(&'a str,&'a str)>,
    start: &'a str,
    turns: I
}

impl<'a, I> Iterator for NetworkIter<'a, I> where I: Iterator<Item=char> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left,right)) = self.net.get(self.start) {
            self.start = match self.turns.next() {
                Some('L') => left,
                Some('R') => right,
                _ => unreachable!()
            };
            Some(self.start)
        } else {
            None
        }
    }
}

pub(crate) struct ParNetworkIter<'a,I> where I: Iterator<Item=char> {
    net: &'a HashMap<&'a str,(&'a str,&'a str)>,
    start: Vec<&'a str>,
    turns: I
}

impl<'a, I> Iterator for ParNetworkIter<'a, I> where I: Iterator<Item=char> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        let turn = self.turns.next();
        self.start = self.start.iter()
            .map(|node|
                match turn {
                    None => unreachable!(),
                    Some('L') => self.net.get(node).map(|(l,_)| *l).unwrap(),
                    Some('R') => self.net.get(node).map(|(_,r)| *r).unwrap(),
                    _ => unreachable!()
                }
            )
            .collect::<Vec<_>>();
        Some(self.start.clone())
    }
}
