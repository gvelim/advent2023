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
    ) -> NetworkIter<'a, impl Iterator<Item=char>> {
        NetworkIter { net: &self, start, turns }
    }

    pub(crate) fn par_iter<'a>(
        &'a self,
        start: &'a [&'a str],
        turns: impl Iterator<Item=char>
    ) -> ParNetworkIter<'a, impl Iterator<Item=char>> {
        ParNetworkIter {
            net: &self, turns,
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
    net: &'a Network<'a>,
    start: &'a str,
    turns: I
}

impl<'a, I> Iterator for NetworkIter<'a, I> where I: Iterator<Item=char> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.turns.next() {
            Some('L') => self.net.net.get(self.start).map(|(l,_)| *l),
            Some('R') => self.net.net.get(self.start).map(|(_,r)| *r),
            _ => unreachable!()
        }.map(|next| {
            self.start = next;
            next
        })
    }
}

pub(crate) struct ParNetworkIter<'a,I> where I: Iterator<Item=char> {
    net: &'a Network<'a>,
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
                    Some('L') => self.net.net.get(node).map(|(l,_)| *l).unwrap(),
                    Some('R') => self.net.net.get(node).map(|(_,r)| *r).unwrap(),
                    _ => unreachable!()
                }
            )
            .collect::<Vec<_>>();
        Some(self.start.clone())
    }
}
