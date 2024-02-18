
use std::collections::{BinaryHeap, HashMap};
use std::io::Read;
use std::ops::Range;
use crate::{
    citymap::CityMap,
    direction::Direction,
    block::*
};
use Direction as D;

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
    fn neighbour_blocks(&'a self, node: CityBlock, rng: &'a Range<usize>) -> impl Iterator<Item=CityBlock> + '_ {
        let CityBlock(pos, dir, step) = node;
        dir.directions()
            // if step < min then move same direction otherwise move all directions
            .filter(move |d|  dir.eq(d) || step >= rng.start)
            // if step == max direction then drop same direction
            .filter(move |d| step < rng.end || dir.ne(d) )
            // extract CityBlocks from valid moves remaining
            .filter_map(move |d|
                self.cmap.move_from(pos, d)
                    .map(|p|
                        CityBlock(p, d, if d == dir {step + 1} else { 1 })
                    )
            )
    }

    pub(crate) fn heat_loss_at_target(&mut self, target: Position, rng: Range<usize>) -> Option<Heat> {
        let mut cost_map = HashMap::<CityBlock,(Heat, Option<CityBlock>)>::new();
        let mut queue = BinaryHeap::<QueuedCityBlock>::new();

        queue.push( QueuedCityBlock(0, CityBlock(self.pos, self.dir, 0)) );
        cost_map.insert(CityBlock(self.pos, self.dir, 0), (0, None));

        while let Some(QueuedCityBlock(heat, node)) = queue.pop() {
            // println!("Popped {:?}",(heat, &node));

            if node.0 == target {
                // self.print_path(node, &cost_map);
                return Some(heat)
            }

            if heat > cost_map.get(&node).unwrap_or(&(Heat::MAX, None)).0 { continue }

            self.neighbour_blocks(node, &rng)
                .for_each(|n| {
                    let CityBlock(p, d, s) = n;
                    let heat_sum = heat + self.cmap[p];
                    // print!("\t({p},{:?},{heat_sum}",d);
                    if heat_sum < cost_map.get(&CityBlock(p, d, s)).unwrap_or(&(Heat::MAX, None)).0 {
                        // println!(",{s}) ✅");
                        cost_map.insert(CityBlock(p, d, s), (heat_sum, Some(node)));
                        queue.push(QueuedCityBlock(heat_sum, CityBlock(p, d, s)));
                    }// else { println!(") ❌") }
                });
            // self.print_path(node, &cost_map);
            // println!("{:?}",queue);
            // let _ = std::io::stdin().read(&mut [0;1]);
        }
        None
    }
    fn print_path(&self, target: CityBlock, cost_map: &HashMap::<CityBlock,(Heat, Option<CityBlock>)>) {

        let mut path: Vec<Option<(Heat, Direction, Step)>> = vec![None; self.cmap.len()];

        let (mut heat, mut parent) = cost_map[&target];
        path[target.0] = Some((heat, target.1, target.2));
        while let Some(n) = parent {
            (heat, parent) = cost_map[&n];
            path[n.0] = Some((heat, n.1, n.2));
        }

        for idx in 0..self.cmap.len() {
            if idx % self.cmap.width() == 0 { println!(); }
            print!("{a}{:2}/{:<3?}:{b:2} |", self.cmap[idx],
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crucible_next() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let mut c = map.get_crucible(0, D::Right);
        assert_eq!(
            Some(102),
            c.heat_loss_at_target(map.len()-1, 1..3)
        );

        let mut c = map.get_crucible(0, D::Right);
        assert_eq!(
            Some(94),
            c.heat_loss_at_target(map.len()-1, 4..10)
        );
    }
    #[test]
    fn test_neighbour_blocks() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let data = [
            ((13,D::Right), vec![(D::Right, 14, 2),(D::Up, 0, 1),(D::Down, 26, 1)]),
            ((13,D::Left), vec![(D::Up, 0, 1),(D::Down, 26, 1)]),
            ((25,D::Left), vec![(D::Left, 24, 2),(D::Up, 12, 1),(D::Down, 38, 1)]),
            ((25,D::Right), vec![(D::Up, 12, 1),(D::Down, 38, 1)]),
            ((168,D::Up),vec![(D::Up, 155, 2),(D::Left, 167, 1)]),
            ((0,D::Up),vec![(D::Right, 1, 1)]),
            ((12,D::Right),vec![(D::Down, 25, 1)]),
            ((156,D::Left),vec![(D::Up, 143, 1)])
        ];

        for ((pos,dir), out) in data.into_iter() {
            let crucible = map.get_crucible(pos, dir);
            let node = CityBlock(pos, dir, 1);
            let iter = crucible.neighbour_blocks(node, &(1..3usize));
            iter.enumerate()
                .inspect(|d| println!("{:?} => {:?}", (pos, dir), d))
                .for_each(|(i,p)|
                    assert_eq!((p.1,p.0,p.2),out[i])
                )
        }
    }
}