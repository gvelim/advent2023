mod network;

use crate::network::Network;
use std::rc::Rc;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8/input.txt").expect("Ops!");
    let (turns, n) = Map::parse(input.as_str());
    let net = Rc::new(n);

    let t = std::time::Instant::now();
    println!("\nPart 1: Steps {:?} - {:?}", net.clone()
        .iter("AAA", turns.chars().cycle())
        .take_while(|node| !(node as &str).eq("ZZZ"))
        .count() + 1,
        (t.elapsed(),Rc::strong_count(&net))
    );

    let t = std::time::Instant::now();
    let a_nodes = net.net
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<std::rc::Rc<[_]>>();

    println!("{:?}",(&a_nodes,Rc::strong_count(&net)));

    let steps = a_nodes.iter()
        .map(|node|
            net.clone()
                .iter(node, turns.chars().cycle())
                .take_while(|node| !node.ends_with("Z"))
                .count() + 1
        )
        .reduce( num::integer::lcm )
        .unwrap();

    println!("Part 2: Steps {:?} - {:?}", steps, (t.elapsed(),Rc::strong_count(&net)));
}

struct Map;
impl Map {
    fn parse(input: &str) -> (&str, Network) {
        let mut split = input.split("\n\n");
        (
            split.next().unwrap(),
            split.next().unwrap()
                .parse::<Network>()
                .unwrap_or_else(|e| panic!("{}",e))
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT_P1: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    static INPUT_P2: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn test_network_lcm() {
        let (turns, n) = Map::parse(INPUT_P2);
        let net = Rc::new(n);

        let a_nodes = net.net
            .keys()
            .filter(|s| s.ends_with('A'))
            .cloned()
            .collect::<Rc<[_]>>();
        println!("{:?}",a_nodes);

        let lcm = a_nodes
            .iter()
            .inspect(|n| print!("{:?} -> ",n))
            .map(|node| {
                let sum = net.clone()
                    .iter(node, turns.chars().cycle())
                    .take_while(|node| !node.ends_with('Z'))
                    .count() + 1;
                println!("Steps {:?}", sum);
                sum
            })
            .reduce(num::integer::lcm)
            .unwrap();

        println!("Total steps: {lcm}");
        assert_eq!(lcm,6)
    }

    #[test]
    fn test_network_traversing() {
        let (turns, net) = Map::parse(INPUT_P1);

        let count = Rc::new(net)
            .iter("AAA", turns.chars().cycle())
            .inspect(|n| println!("{:?}",n))
            .take_while(|node| (node as &str).ne("ZZZ") )
            .count() + 1;

        assert_eq!(count,6)
    }

    #[test]
    fn test_parse_directions() {
        let (turns,_) = Map::parse(INPUT_P1);
        let out = turns.chars().cycle().take(5).collect::<Vec<_>>();
        println!("{:?}",out);
        assert_eq!(
            vec!['L', 'L', 'R', 'L', 'L'],
            out
        );
    }
}
