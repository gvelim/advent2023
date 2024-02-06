use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::{citymap::{CityMap,Heat,Position}, direction::Direction};
use Direction as D;

const STEPS: usize = 3;

#[derive(Debug,Eq)]
struct Node(Position, Direction, Heat, usize);
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2).then_with(|| self.1.cmp(&other.1))
    }
}
impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
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
        let mut history = vec![(Heat::MAX,false,None); self.cmap.len()];
        let mut heat_cost = BinaryHeap::<Node>::new();
        heat_cost.push( Node(self.pos, self.dir, self.heat, 1));

        while let Some(block) = heat_cost.pop() {
            // println!("Popped {:?}",block);
            let Node(pos, dir, heat, steps) = block;

            if pos == target {
                let mut path = std::collections::HashSet::<Position>::new();
                path.insert(target);
                let mut par: Option<Position> = history[target].2;
                while let Some(p) = par {
                    path.insert(p);
                    par = history[p].2;
                }

                for idx in 0..self.cmap.len() {
                    if idx % self.cmap.width() == 0 { println!(); }
                    print!("{:2}/{:<3?}{}", self.cmap[idx], history[idx].0,
                        if path.contains(&idx) { '*' } else { ' ' }
                    );
                }
                println!();
                return Some(history[pos].0)
            }

            self.get_neighbours(pos,dir)
                .filter(|(d,_)|
                    !(steps >= STEPS && dir.eq(d))
                )
                .for_each(|(d,p)| {
                    let cost = heat + self.cmap[p];
                    if cost < history[p].0 {
                        let s = if d == dir { steps + 1 } else { 1 };
                        history[p].0 = cost;
                        history[p].2 = Some(pos);
                        heat_cost.push(Node(p, d, cost, s));
                    }
                });
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