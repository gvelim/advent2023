mod directions;
mod network;

use crate::{
    directions::*,
    network::Network
};

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        _ if a == b => a,
        ((_, 0), _) => a,
        ((0, _), _) => b,
        (_, (0, 1) | (1, 0)) => gcd(a >> 1, b),
        (_, (0, 0)) => gcd(a >> 1, b >> 1) << 1,
        (_, (1, 1)) => {
            let (a, b) = (a.min(b), a.max(b));
            gcd((b - a) >> 1, a)
        }
        _ => unreachable!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day8/input.txt").expect("Ops!");
    let split = input.split("\n\n").next().unwrap();
    let turns_len = split.len();
    let mut turns = Directions::parse(split);
    let mut net = Network::parse(input.as_str());

    let mut run_part = |start: &str, net: &mut Network, cmp: fn(&str) -> bool| {
        net.iter(start, &mut turns)
            // .inspect(|n| print!("{:?},",n))
            .take_while(|node| cmp(node))
            .count() + 1
    };

    println!("\nPart 1: Count {:?}",
             run_part("AAA", &mut net, |n| !n.eq("ZZZ"))
    );

    let mut a_nodes = net.net.keys().filter(|s| s.ends_with('A')).copied().collect::<Vec<_>>();
    a_nodes.sort();
    println!("{:?}",(&a_nodes,&turns_len));

    let count = a_nodes.iter()
        .inspect(|n| print!("{:?} -> ",n))
        .map(|node| {
            let sum = run_part(node, &mut net, |n| !n.ends_with('Z'));
            println!("Part 2: Count {:?}", sum);
            sum
        })
        .reduce(|a,b| (b*a)/gcd(b,a));

    println!("\nPart 2: Count {:?}",count);
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;
    use super::Directions::*;

    static INPUT_P1: &str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
    static INPUT_P2: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn test_network_lcm() {
        let mut split = INPUT_P2.split("\n\n");
        let mut turns = Directions::parse(split.next().unwrap());
        let mut net = Network::parse(INPUT_P2);

        let mut run_part = |start: &str, net: &mut Network, cmp: fn(&str) -> bool| {
            net.iter(start, &mut turns)
                // .inspect(|n| print!("{:?},",n))
                .take_while(|node| cmp(node))
                .count() + 1
        };

        let mut a_nodes = net.net.keys().filter(|s| s.ends_with('A')).copied().collect::<Vec<_>>();
        a_nodes.sort();
        println!("{:?}",a_nodes);

        a_nodes.iter()
            .inspect(|n| print!("{:?} -> ",n))
            .for_each(|node| {
                let sum = run_part(node, &mut net, |n| !n.ends_with('Z'));
                println!("Part 2: Count {:?}", sum)
            });

        assert_eq!(6,6)
    }

    #[test]
    fn test_network_parallel_traversing() {
        let mut split = INPUT_P2.split("\n\n");
        let turns = Directions::parse(split.next().unwrap());
        let mut net = Network::parse(INPUT_P2);

        let mut a_nodes = net.net.keys().filter(|s| s.ends_with('A')).copied().collect::<Vec<_>>();
        a_nodes.sort();
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
        let mut split = INPUT_P1.split("\n\n");
        let turns = Directions::parse(split.next().unwrap());
        let mut net = Network::parse(INPUT_P1);

        let count = net.iter("AAA", turns)
            .inspect(|n| println!("{:?}",n))
            .take_while(|node| node.ne(&"ZZZ") )
            .count() + 1;



        assert_eq!(count,6)
    }

    #[test]
    fn test_parse_directions() {
        let mut split = INPUT_P1.split("\n\n");
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
        let net = Network::parse(INPUT_P1);

        println!("{:?}",net);
        assert_eq!(
            Network { net: HashMap::from([
                ("ZZZ", ("ZZZ", "ZZZ")), ("AAA", ("BBB", "BBB")), ("BBB", ("AAA", "ZZZ"))
            ])},
            net
        )
    }
}