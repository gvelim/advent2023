use std::collections::HashMap;
use crate::directions::Directions;

#[derive(Debug,PartialEq)]
pub(crate) struct Network<'a> {
    pub(crate) net: HashMap<&'a str,(&'a str, &'a str)>,
}
impl Network<'_> {

    pub(crate) fn iter<'a>(&'a mut self, start: &'a str, turns: impl Iterator<Item=Directions>) -> MapIter<'a, impl Iterator<Item=Directions>> {
        MapIter { net: &self.net, start, turns }
    }

    pub(crate)  fn parse(s: &str) -> Network<'_> {
        let mut split = s.split("\n\n").skip(1);
        Network {
            net: split.next().unwrap().lines()
                .map(|line| {
                    let mut iter = line.split([' ', '=', '(', ')', ','])
                        .filter(|&s| !s.is_empty());
                    (iter.next().unwrap(), (iter.next().unwrap(),iter.next().unwrap()))
                })
                .collect::<HashMap<&str,(&str,&str)>>()
        }
    }
}

pub(crate) struct MapIter<'a,I> where I: Iterator<Item=Directions> {
    net: &'a HashMap<&'a str,(&'a str,&'a str)>,
    start: &'a str,
    turns: I
}
impl<'a, I> Iterator for MapIter<'a, I> where I: Iterator<Item=Directions> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left,right)) = self.net.get(self.start) {
            self.start = match self.turns.next() {
                None => unreachable!(),
                Some(Directions::Left) => left,
                Some(Directions::Right) => right
            };
            Some(self.start)
        } else {
            None
        }
    }
}
