mod directions;
mod network;

use crate::{
    directions::*,
    network::Network
};

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8/input.txt").expect("Ops!");
    let mut split = input.split("\n\n");
    let turns = Directions::parse(split.next().unwrap());
    let mut net = Network::parse(input.as_str());

    let count = net.iter("AAA", turns)
        .inspect(|n| print!("{:?},",n))
        .take_while(|node| node.ne(&"ZZZ") )
        .count() + 1;

    println!("\nPart 1: Count {}",count);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;
    use super::Directions::*;

    static INPUT: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_network_traversing() {
        let mut split = INPUT.split("\n\n");
        let turns = Directions::parse(split.next().unwrap());
        let mut net = Network::parse(INPUT);

        let count = net.iter("AAA", turns)
            .inspect(|n| println!("{:?}",n))
            .take_while(|node| node.ne(&"ZZZ") )
            .count() + 1;

        assert_eq!(count,6)
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
        let net = Network::parse(INPUT);

        println!("{:?}",net);
        assert_eq!(
            Network { net: HashMap::from([
                ("ZZZ", ("ZZZ", "ZZZ")), ("AAA", ("BBB", "BBB")), ("BBB", ("AAA", "ZZZ"))
            ])},
            net
        )
    }
}