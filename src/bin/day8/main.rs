mod directions;
mod node;

use std::collections::HashMap;
use std::str::FromStr;

use crate::{ directions::*, node::* };

fn main() {

}

struct MapIter<'a,I> where I: Iterator<Item=directions::Directions> {
    net: &'a HashMap<String,(String,String)>,
    seed: &'a String,
    dir: I
}
impl<'a, I> Iterator for MapIter<'a, I> where I: Iterator<Item=directions::Directions> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((left,right)) = self.net.get(self.seed) {
            let dir = self.dir.next();
            print!("{:?}",dir);
            self.seed = match dir {
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

struct Map {
    network: HashMap<String,(String,String)>,
}
impl Map {
    fn get_iter<'a>(&'a mut self, seed: &'a String, dir: impl Iterator<Item=Directions>) -> MapIter<'a, impl Iterator<Item=Directions>> {
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

#[cfg(test)]
mod test {
    use super::*;
    use super::Directions::*;

    static INPUT: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_network_traversing() {
        let mut split = INPUT.split("\n\n");
        let turns = Directions::parse(split.next().unwrap());
        let mut net = INPUT.parse::<Map>().expect("Ops");

        let count = net.get_iter(&"AAA".to_string(), turns)
            .inspect(|n| println!("{:?}",n))
            .take_while(|node| node.ne(&"ZZZ") )
            .count();

        assert_eq!(count+1,6)
    }

    #[test]
    fn test_parse_directions() {
        let mut split = INPUT.split("\n\n");
        let turns = Directions::parse(split.next().unwrap());
        let out = turns.take(5).collect::<Vec<_>>();
        println!("{:?}",out);
        assert_eq!(
            vec![Left, Left, Right, Left, Left],
            out
        );
    }
    #[test]
    fn test_parse_nodes() {
        let mut split = INPUT.split("\n\n").skip(1);
        let node = split.next().unwrap().lines()
            .map(|line| line.parse::<Node>().expect("Ops"))
            .next()
            .expect("Ops!");

        println!("{:?}",node);
        assert_eq!(
            Node { name: "AAA".to_string(), left: "BBB".to_string(), right: "BBB".to_string() },
            node
        );
    }
}