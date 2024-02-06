use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::{citymap::{CityMap,Heat,Position}, direction::Direction};
use Direction as D;

#[derive(Debug,Ord,Eq)]
struct Node(Position, Direction, Heat, usize);
impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.2.cmp(&self.2))
    }
}

#[derive(Debug)]
pub(crate) struct Crucible<'a> {
    cmap: &'a CityMap,
    pos: Position,
    dir: Direction,
    heat: Heat,
}

impl<'a> Crucible<'a> {
    pub(crate) fn new(map: &CityMap, pos: Position, dir: Direction) -> Crucible {
        Crucible { cmap: map, pos, dir, heat: map[pos] }
    }
    pub(crate) fn get_neighbours(&self, pos: Position, dir: Direction) -> impl Iterator<Item=(Direction, Position)> + '_ {
        match dir {
            D::Up => [D::Up, D::Left, D::Right],
            D::Right => [D::Right, D::Up, D::Down],
            D::Down => [D::Down, D::Left, D::Right],
            D::Left => [D::Left, D::Up, D::Down],
        }
            .into_iter()
            .filter_map(move |dir|
                self.cmap.step_onto(pos, dir).map(|p| (dir, p))
            )
    }
    pub(crate) fn heat_to_target_block(&mut self, target: Position) -> Option<Heat> {
        let mut history = vec![(u8::MAX,false,None); self.cmap.len()];
        let mut heat_cost =
            BinaryHeap::<Node>::from([Node(self.pos, self.dir, self.heat, 1)]);

        while let Some(cblock) = heat_cost.pop() {
            // println!("Popped {:?}",cblock);
            let Node(pos, dir, heat, steps) = cblock;

            if pos == target {
                for idx in 0..self.cmap.len() {
                    if idx % 13 == 0 { println!(); }
                    print!("{:2}/{:<3?}{}", self.cmap[idx], history[idx].0,
                           match history[idx].2 {
                               None => '*',
                               Some(D::Right) => 'R',
                               Some(D::Left) => 'L',
                               Some(D::Up) => 'U',
                               Some(D::Down) => 'D'
                           });
                }
                println!();
                return Some(history[pos].0)
            }

            self.get_neighbours(pos,dir)
                .filter(|next|
                    !(steps > 2 && next.0 == dir)
                )
                .for_each(|next| {
                    if !history[next.1].1 {
                        let cost = self.cmap[next.1] + heat;
                        if history[next.1].0 > cost {
                            let s = if next.0 == dir { steps + 1 } else { 1 };
                            history[next.1].0 = cost;
                            history[next.1].2 = Some(dir);
                            // println!("\t{:?}", Node(next.1, next.0, cost, s));
                            heat_cost.push(Node(next.1, next.0, cost, s));
                        }
                    }
                });

            history[pos].1 = true;
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
        println!("{:?}",c.heat_to_target_block(168));
    }
    #[test]
    fn test_neighbour_blocks() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let data = [
            ((13,D::Right), vec![(D::Right, 14),(D::Up, 0),(D::Down, 26)]),
            ((25,D::Left), vec![(D::Left, 24),(D::Up, 12),(D::Down, 38)]),
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

}