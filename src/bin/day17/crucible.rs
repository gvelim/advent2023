use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::Read;
use crate::{citymap::{CityMap,Heat,Position}, direction::Direction};
use Direction as D;

const STEPS: usize = 3;

type Step = usize;

#[derive(Debug, Eq, PartialEq)]
struct Node(Position, Direction, Heat);
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    pub(crate) fn get_neighbours(&self, pos: Position, dir: Direction) -> impl Iterator<Item=(Direction, Position)> + '_ {
        dir.directions()
            .filter_map(move |d|
                self.cmap.step(pos, d).map(|p| (d, p))
            )
    }
    pub(crate) fn look_ahead(&self, pos: Position, dir: Direction, steps: Step) -> impl Iterator<Item=(Direction, Position)> + '_ {
        let mut np = pos;
        (0..steps).filter_map(move |_| {
            self.cmap.step(np,dir).map(|p|{ np = p; (dir,p)})
        })
    }

    pub(crate) fn heat_to_target_block(&mut self, target: Position) -> Option<Heat> {
        let mut dist = vec![(Heat::MAX, None, None, 0); self.cmap.len()];
        let mut queue = BinaryHeap::<Node>::new();

        let print_citymap = |pos: Position, history: &Vec<(Heat, Option<Position>, Option<Direction>, Step)> | {
            let mut path = std::collections::HashSet::<Position>::new();
            path.insert(pos);
            let mut par: Option<Position> = history[pos].1;
            while let Some(p) = par {
                path.insert(p);
                par = history[p].1;
            }

            for idx in 0..self.cmap.len() {
                if idx % self.cmap.width() == 0 { println!(); }
                print!("{a:1}{:2}/{:<3?}", self.cmap[idx], history[idx].0,
                       a=if path.contains(&idx) {
                           match history[idx].2 {
                               None => '◼', Some(D::Up) => '▲', Some(D::Down) => '▼',
                               Some(D::Left) => '◀', Some(D::Right) => '▶',
                           }
                       } else { ' ' }
                );
            }
            println!();
        };

        queue.push( Node(self.pos, self.dir, 0) );
        dist[self.pos] = (0, None, None, 2);

        while let Some(block) = queue.pop() {
            // println!("Popped {:?}",block);
            let Node(pos, dir, heat) = block;

            if pos == target {
                print_citymap(pos,&dist);
                return Some(heat)
            }

            if heat > dist[pos].0 { continue }

            let steps = dist[pos].3;
            self.get_neighbours(pos,dir)
                .filter(|(d,_)|
                    !(steps == STEPS && dir.eq(d))
                )
                .for_each(|(d,p)| {
                    let heat_sum = heat + self.cmap[p];
                    // println!("\t{:?}",(d, p, heat_sum));
                    if heat_sum < dist[p].0 {
                        dist[p] = (heat_sum, Some(pos), Some(d), if d == dir { steps + 1 } else { 1 });
                        queue.push(Node(p, d, heat_sum));
                    }
                });
            // print_citymap(pos, &dist);
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
        println!("{:?}",c.heat_to_target_block(map.len()-1));
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