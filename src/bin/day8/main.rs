mod network;

use crate::network::Network;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8/input.txt").expect("Ops!");
    let (mut turns, net) = Map::parse(input.as_str());

    let t = std::time::Instant::now();
    println!("\nPart 1: Steps {:?} - {:?}",
             net.iter("AAA", &mut turns)
                 .take_while(|node| !node.eq(&"ZZZ")).count() + 1,
             t.elapsed()
    );

    let t = std::time::Instant::now();
    let a_nodes = net.net.keys().filter(|s| s.ends_with('A')).copied().collect::<std::rc::Rc<[_]>>();
    println!("{:?}",a_nodes);

    let steps = a_nodes.iter()
        .map(|node|
            net.iter(node, &mut turns)
                .take_while(|node| !node.ends_with(&"Z"))
                .count() + 1
        )
        .reduce( num::integer::lcm )
        .unwrap();

    println!("Part 2: Steps {:?} - {:?}", steps, t.elapsed());
}

struct Map;
impl Map {
    fn parse(input: &str) -> (impl Iterator<Item=char> + '_, Network) {
        let mut split = input.split("\n\n");
        (
            split.next().unwrap().chars().cycle(),
            Network::parse(split.next().unwrap())
        )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    static INPUT_P1: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    static INPUT_P2: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn test_network_lcm() {
        let (mut turns, net) = Map::parse(INPUT_P2);

        let a_nodes = net.net.keys().filter(|s| s.ends_with('A')).collect::<std::rc::Rc<[_]>>();
        println!("{:?}",a_nodes);

        let lcm = a_nodes.iter()
            .inspect(|&&&n| print!("{:?} -> ",n))
            .map(|&&node| {
                let sum = net.iter(node, &mut turns)
                    .take_while(|node| !node.ends_with(&"Z"))
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
    fn test_network_parallel_traversing() {
        let (turns, net) = Map::parse(INPUT_P2);

        let a_nodes = net.net.keys().filter(|s| s.ends_with('A')).copied().collect::<Vec<_>>();
        println!("{:?}",a_nodes);

        let count = net.par_iter(&a_nodes, turns)
            .inspect(|n| println!("{:?}",n))
            .take_while(|nodes|
                !nodes.iter().all(|node| node.ends_with("Z"))
            )
            .count() + 1;

        assert_eq!(count,6)
    }

    #[test]
    fn test_network_traversing() {
        let (turns, net) = Map::parse(INPUT_P1);

        let count = net.iter("AAA", turns)
            .inspect(|n| println!("{:?}",n))
            .take_while(|node| node.ne(&"ZZZ") )
            .count() + 1;

        assert_eq!(count,6)
    }

    #[test]
    fn test_parse_directions() {
        let (turns,_) = Map::parse(INPUT_P1);
        let out = turns.take(5).collect::<Vec<_>>();
        println!("{:?}",out);
        assert_eq!(
            vec!['L', 'L', 'R', 'L', 'L'],
            out
        );
    }
    #[test]
    fn test_parse_nodes() {
        let (_,net) = Map::parse(INPUT_P1);

        println!("{:?}",net);
        assert_eq!(
            Network { net: HashMap::from([
                ("ZZZ", ("ZZZ", "ZZZ")), ("AAA", ("BBB", "BBB")), ("BBB", ("AAA", "ZZZ"))
            ])},
            net
        )
    }
}