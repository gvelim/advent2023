
use std::collections::{BinaryHeap, HashMap};
// use std::io::Read;
use std::ops::Range;
use crate::{
    citymap::CityMap,
    direction::Direction,
    block::*,
    path::CityMapPath
};

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
    fn neighbour_blocks(&'a self, node: CityBlock, rng: &'a Range<Position>) -> impl Iterator<Item=CityBlock> + '_ {
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

    pub(crate) fn find_path_to(&mut self, target: Position, rng: Range<Position>) -> Option<CityMapPath> {
        let mut cost_map = HashMap::<CityBlock,(Heat, Option<CityBlock>)>::new();
        let mut queue = BinaryHeap::<QueuedCityBlock>::new();
        // push starting conditions of zero heat, zero steps
        queue.push( QueuedCityBlock(0, CityBlock(self.pos, self.dir, 0)) );
        cost_map.insert(CityBlock(self.pos, self.dir, 0), (0, None));
        // pull the next block with the least heat cost from the queue
        while let Some(QueuedCityBlock(heat, block)) = queue.pop() {
            // is this block our target ?
            if block.0 == target {
                // yes, return the path cost map with the starting block for traversing it
                return Some(CityMapPath::new(cost_map, block))
            }
            // get all feasible neighbouring blocks given the constraints
            self.neighbour_blocks(block, &rng)
                .for_each(|neighbour| {
                    // calculate cost if we are to move to this neighbour
                    let heat_sum = heat + self.cmap[neighbour.0];
                    // is the cost higher than previously found ? if not, store it
                    if heat_sum < cost_map.get(&neighbour).unwrap_or(&(Heat::MAX, None)).0 {
                        // remember the heat cost at this block along the block we stepped from
                        cost_map.insert(neighbour, (heat_sum, Some(block)));
                        // push neighbouring block to priority queue for processing
                        queue.push(QueuedCityBlock(heat_sum, neighbour));
                    }
                });
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction as D;

    #[test]
    fn test_crucible_next() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        let test_ranges = |result: Heat, rng:Range<Position>| {
            let mut c = map.get_crucible(0, D::Right);
            c.find_path_to(map.len()-1, rng)
                .map(|path| {
                    map.display_path(&path);
                    assert_eq!(
                        result,
                        path.total_heat_loss()
                    );
                })
                .or_else(|| panic!("Path not found"));
        };

        test_ranges(102,1..3);
        test_ranges(94,4..10);
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