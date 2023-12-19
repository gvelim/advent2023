#![feature(slice_group_by)]

mod field;
mod direction;
mod elf;

use crate::field::Field;
use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").expect("Can't read input");
    let f = Field::parse(input.as_str(),'S');

    let mut elf = f.get_walking_elf(None);

    let dirs = elf.valid_directions();
    println!("Available directions {:?}",dirs);
    elf.dir = if dirs.is_empty() { panic!("Ops! cannot find valid direction to go!") } else { dirs[0] };

    let steps = elf
        .take_while(|(p, _)| 'S'.ne(p))
        .map(|(_,pos)| pos)
        .collect::<Vec<_>>();
    let count = &steps.iter().count();
    println!("Part 1 : Total steps: {}, furthest away: {}", count, count/2 + 1);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::direction::Direction::{Down, Left, Right, Up};
    static INPUT_PART1: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
    static INPUT_PART2: &str = "...........\n\
                                .S-------7.\n\
                                .|F-----7|.\n\
                                .||.....||.\n\
                                .||.....||.\n\
                                .|L-7.F-J|.\n\
                                .|..|.|..|.\n\
                                .L--J.L--J.\n\
                                ...........";
    #[test]
    fn test_count_area() {
        let f = Field::parse(INPUT_PART2, 'S');
        let mut elf = f.get_walking_elf(None);

        let dirs = elf.valid_directions();
        println!("Available directions {:?}",dirs);
        elf.dir = if dirs.is_empty() { panic!("Ops! cannot find valid direction to go!") } else { dirs[0] };

        let mut steps = elf
            .take_while(|(p, _)| 'S'.ne(p))
            .collect::<Vec<_>>();

        steps.push(('S', f.start));
        steps.sort_by(|(_,a),(_,b)|
            match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                cmp => cmp
            });

        let tiles = steps.group_by_mut(|(_,a),(_,b)| a.1 == b.1 )
            .inspect(|c| println!("Group: {:?}",c))
            .map(|d|{
                let mut dash = 0;
                d.iter_mut()
                    .filter_map(|pipe|
                        if '-'.ne(&pipe.0) {
                            pipe.1.0 -= dash;
                            Some(pipe)
                        } else {
                            dash += 1;
                            None
                        }
                    )
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .inspect(|c| print!("Pair: {:?}",c))
                    .map(|pair| {
                        let [(_,a),(_,b)] = pair else { todo!() };
                        b.0 - a.0 - 1
                    })
                    .inspect(|c| println!("Sum: {:?}",c))
                    .sum::<usize>()
            })
            .inspect(|c| println!("Sum: {:?}\n",c))
            .sum::<usize>();
    }
    #[test]
    fn test_pipe_waking() {
        let f = Field::parse(INPUT_PART1, 'S');
        let elf = f.get_walking_elf(None);

        assert_eq!(
            elf.take_while(|(pipe,_)| 'S'.ne(pipe))
                .inspect(|p| println!("{:?},",p))
                .map(|(p,_)|p)
                .collect::<Vec<_>>(),
            ['J', 'F', 'J', 'F', '7', '|', 'L', '7', 'J', '-', '-', 'F', 'J', 'L', '|']
        );
    }
    #[test]
    fn test_direction() -> Result<(),()> {
        let f = Field::parse(INPUT_PART1, 'S');

        let mut dir = Up.pipe_exit( f.get_pipe((2, 0)).unwrap()  ).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit(f.get_pipe((3, 0)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get_pipe((3, 1)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get_pipe((3, 2)).unwrap()).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit( f.get_pipe((4, 2)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit( f.get_pipe((4, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((3, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((2, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((1, 3)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        Ok(())
    }
    #[test]
    fn test_boundaries() {
        let f = Field::parse(INPUT_PART1, 'S');
        
        assert_eq!(Some('S'), f.get_pipe(f.start));
        assert_eq!(Some('7'), f.get_pipe((3, 0)));
        assert_eq!(Some('F'), f.get_pipe((2, 0)));
        assert_eq!(Some('J'), f.get_pipe((2, 1)));
        assert_eq!(Some('.'), f.get_pipe((2, 2)));
        assert_eq!(Some('J'), f.get_pipe((4, 3)));
        assert_eq!(Some('.'), f.get_pipe((4, 4)));
        assert_eq!(None, f.get_pipe((8, 3)));
        assert_eq!(None, f.get_pipe((3, 6)));
        assert_eq!(None, f.get_pipe((8, 6)));
    }
    #[test]
    fn test_parse_map() {
        let f = Field::parse(INPUT_PART1, 'S');
        assert_eq!(
            f,
            Field {
                data: vec!['.', '.', 'F', '7', '.', '.', 'F', 'J', '|', '.', 'S', 'J', '.', 'L', '7', '|', 'F', '-', '-', 'J', 'L', 'J', '.', '.', '.'],
                width: 5,
                start: (0,2)
            }
        );
    }

}
