mod directions;
mod node;
mod map;

use crate::{
    directions::*,
    node::*,
    map::Map
};

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8/input.txt").expect("Ops!");
    let mut split = input.split("\n\n");
    let turns = Directions::parse(split.next().unwrap());
    let mut net = input.parse::<Map>().expect("Ops");

    let count = net.iter(&"AAA".to_string(), turns)
        .inspect(|n| print!("{:?},",n))
        .take_while(|node| node.ne(&"ZZZ") )
        .count();

    println!("\nPart 1: Count {}",count+1);
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

        let count = net.iter(&"AAA".to_string(), turns)
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