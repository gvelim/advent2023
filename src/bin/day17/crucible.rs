use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::Read;
use std::ops::Range;
use crate::{citymap::{CityMap,Heat,Position}, direction::Direction};
use Direction as D;

const STEPS: usize = 3;

type Step = usize;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Copy, Clone)]
struct Node(Position, Direction, Step);

#[derive(Debug, Eq)]
struct Block(Heat,Node);
impl PartialEq<Self> for Block {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd<Self> for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.2.cmp(&other.1.2))
    }
}

#[derive(Debug)]
pub(crate) struct Crucible<'a> {
    cmap: &'a CityMap,
    pos: Position,
    dir: Direction
}

impl<'a> Crucible<'a> {
    pub(crate) fn new(map: &CityMap, pos: Position, dir: Direction) -> Crucible {
        Crucible { cmap: map, pos, dir }
    }
    fn get_neighbours(&self, pos: Position, dir: Direction) -> impl Iterator<Item=(Direction, Position)> + '_ {
        dir.directions()
            .filter_map(move |d|
                self.cmap.move_from(pos, d).map(|p| (d, p))
            )
    }

    fn get_neighbours_2(&self, pos: Position, dir: Direction, step: Step, min: usize) -> impl Iterator<Item=(Direction, Position, Step)> + '_ {
        dir.directions()
            .filter(move |d| (step < min && *d == dir) || step >= min)
            .filter_map(move |d|
                self.cmap.move_from(pos, d)
                    .map(|p|
                        (d, p, if d == dir {step + 1} else { 1 })
                    )
            )
    }

    fn look_ahead(&self, pos: Position, dir: Direction, steps: Step) -> impl Iterator<Item=(Direction, Position)> + '_ {
        let mut np = pos;
        (0..steps).filter_map(move |_| {
            self.cmap.move_from(np, dir).map(|p|{ np = p; (dir, p)})
        })
    }

    fn print_path(&self, target: Node, cost_map: &HashMap::<Node,(Heat, Option<Node>)>) {

        let mut path: Vec<Option<(Heat, Direction, Step)>> = vec![None; self.cmap.len()];

        let (mut heat, mut parent) = cost_map[&target];
        path[target.0] = Some((heat, target.1, target.2));
        while let Some(n) = parent {
            (heat, parent) = cost_map[&n];
            path[n.0] = Some((heat, n.1, n.2));
        }

        for idx in 0..self.cmap.len() {
            if idx % self.cmap.width() == 0 { println!(); }
            print!("{a}{:2}/{:<3?}:{b} |", self.cmap[idx],
                   path[idx].map(|(h,..)| h).unwrap_or(0),
                   a=if path[idx].is_some() {
                       match path[idx].map(|(_,d,_)| d) {
                           None => '◼', Some(D::Up) => '▲', Some(D::Down) => '▼',
                           Some(D::Left) => '◀', Some(D::Right) => '▶',
                       }
                   } else { ' ' },
                   b=path[idx].map(|(..,s)| s).unwrap_or(0)
            );
        }
        println!();
    }

    pub(crate) fn heat_to_target_block(&mut self, target: Position, rng: Range<usize>) -> Option<Heat> {
        let mut cost_map = HashMap::<Node,(Heat, Option<Node>)>::new();
        let mut queue = BinaryHeap::<Block>::new();

        queue.push( Block(0, Node(self.pos, self.dir, 0)) );
        cost_map.insert(Node(self.pos, self.dir, 0), (0, None));

        while let Some(Block(heat, node)) = queue.pop() {
            // println!("Popped {:?}",(heat, &node));

            if node.0 == target {
                // self.print_path(node, &cost_map);
                return Some(heat)
            }

            if heat > cost_map.get(&node).unwrap_or(&(Heat::MAX, None)).0 { continue }

            let Node(pos, dir , steps) = node;
            self.get_neighbours_2(pos, dir, steps, rng.start)
                .filter(|(d,..)|
                    !(steps == rng.end && dir.eq(d))
                )
                .for_each(|(d,p, s)| {
                    let heat_sum = heat + self.cmap[p];
                    // print!("\t({p},{:?},{heat_sum}",d);
                    if heat_sum < cost_map.get(&Node(p, d, s)).unwrap_or(&(Heat::MAX, None)).0 {
                        // println!(",{s}) ✅");
                        cost_map.insert(Node(p, d, s), (heat_sum, Some(node)));
                        queue.push(Block(heat_sum, Node(p, d, s)));
                    }// else { println!(") ❌") }
                });
            // self.print_path(node, &cost_map);
            // println!("{:?}",queue);
            // let _ = std::io::stdin().read(&mut [0;1]);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crucible_next() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let mut c = map.get_crucible(0, D::Right);
        println!("{:?}",c.heat_to_target_block(map.len()-1, 4..10));
    }
    #[test]
    fn test_neighbour_blocks() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let data = [
            ((13,D::Right), vec![(D::Right, 14),(D::Up, 0),(D::Down, 26)]),
            ((13,D::Left), vec![(D::Up, 0),(D::Down, 26)]),
            ((25,D::Left), vec![(D::Left, 24),(D::Up, 12),(D::Down, 38)]),
            ((25,D::Right), vec![(D::Up, 12),(D::Down, 38)]),
            ((168,D::Up),vec![(D::Up, 155),(D::Left, 167)]),
            ((0,D::Up),vec![(D::Right, 1)]),
            ((12,D::Right),vec![(D::Down, 25)]),
            ((156,D::Left),vec![(D::Up, 143)])
        ];

        for ((inp,dir), out) in data.into_iter() {
            let crucible = map.get_crucible(inp, dir);
            let iter = crucible.get_neighbours(inp,dir);
            iter.enumerate()
                .inspect(|d| println!("{:?} => {:?}",(inp,dir), d))
                .for_each(|(i,p)|
                    assert_eq!(p,out[i])
                )
        }
    }
    #[test]
    fn test_look_ahead() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let data = [
            ((58, D::Left), vec![(D::Left, 57), (D::Left, 56), (D::Left, 55)]),
            ((58, D::Right), vec![(D::Right, 59), (D::Right, 60), (D::Right, 61)]),
            ((58, D::Up), vec![(D::Up, 45), (D::Up, 32), (D::Up, 19)]),
            ((58, D::Down), vec![(D::Down, 71), (D::Down, 84), (D::Down, 97)]),
        ];

        for ((inp, dir), out) in data.into_iter() {
            let crucible = map.get_crucible(inp, dir);
            let iter = crucible.look_ahead(inp, dir, STEPS);
            iter.enumerate()
                .inspect(|d| println!("{:?} => {:?}", (inp, dir), d))
                .for_each(|(i, p)| {
                    assert_eq!(p, out[i])
                })
        }
    }
}