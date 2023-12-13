use std::collections::HashMap;
use std::str::FromStr;
use crate::directions::Directions;
use crate::node::Node;

pub(crate) struct Map {
    network: HashMap<String,(String,String)>,
}
impl Map {
    pub(crate) fn iter<'a>(&'a mut self, seed: &'a String, dir: impl Iterator<Item=Directions>) -> MapIter<'a, impl Iterator<Item=Directions>> {
        MapIter {
            net: &self.network, seed, dir
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n\n").skip(1);
        let network = split.next().unwrap().lines()
            .map(|line| line.parse::<Node>().expect("Ops"))
            .map(|node| (node.name, (node.left,node.right)))
            .collect::<HashMap<String,(String,String)>>();

        Ok(Map { network })
    }
}

pub(crate) struct MapIter<'a,I> where I: Iterator<Item=Directions> {
    net: &'a HashMap<String,(String,String)>,
    seed: &'a String,
    dir: I
}
impl<'a, I> Iterator for MapIter<'a, I> where I: Iterator<Item=Directions> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left,right)) = self.net.get(self.seed) {
            self.seed = match self.dir.next() {
                None => unreachable!(),
                Some(Directions::Left) => left,
                Some(Directions::Right) => right
            };
            Some(self.seed)
        } else {
            None
        }
    }
}
