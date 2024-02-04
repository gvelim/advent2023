use crate::{citymap::CityMap, direction::Direction};
use Direction as D;

pub(crate) type Position = usize;

#[derive(Debug)]
pub(crate) struct Crucible<'a> {
    map: &'a CityMap,
    pos: Position,
    dir: Direction,
    heat: usize,
    steps: u8
}

impl<'a> Crucible<'a> {
    pub(crate) fn new(map: &CityMap, pos: Position, dir: D) -> Crucible {
        Crucible { map, pos, dir, heat: map.map[pos] as usize, steps:0 }
    }
    pub(crate) fn get_neighbours(&self) -> impl Iterator<Item=(Direction, Position)> + '_ {
        match self.dir {
            D::Up => [D::Up, D::Left, D::Right],
            D::Right => [D::Right, D::Up, D::Down],
            D::Down => [D::Down, D::Left, D::Right],
            D::Left => [D::Left, D::Up, D::Down],
        }
            .into_iter()
            .filter_map(|dir|
                self.map.step_onto(self.pos,dir).map(|p| (dir,p))
            )
    }
}

impl Iterator for Crucible<'_> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self
            // get heat from neighbour city-blocks
            .get_neighbours()
            // calculate heat cost per neighbour
            .filter(|block|
                if !(self.steps > 2 && block.0 == self.dir) { true }
                else { print!("{:?}❌, ",block); false }
            )
            .map(|block|
                (block, self.heat + self.map.map[block.1] as usize)
            )
            .inspect(|d| print!("{:?}👀, ",d))
            // pick block where heat_loss = min(block1, block2, block3)
            .min_by_key(|&(_,a)| a)
            .map(|block|{
                // accumulate heat
                self.heat = block.1;
                // have we moved more than once in the same direction ?
                self.steps = if block.0.0 == self.dir { self.steps + 1 } else { 1 };
                print!("<= Min:{:?}, Cost:{:?}, Steps:{:?}",block,self.heat,self.steps);
                // move onto city block
                (self.dir, self.pos) = block.0;
                self.pos
            })
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
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
        println!("Move to -> {:?}",c.next());
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
            let iter = crucible.get_neighbours();
            iter.enumerate()
                .inspect(|d| println!("{:?} => {:?}",(inp,dir), d))
                .for_each(|(i,p)|
                    assert_eq!(p,out[i])
                )
        }
    }

}