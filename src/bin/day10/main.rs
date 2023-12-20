#![feature(slice_group_by)]
#![feature(iter_collect_into)]

mod direction;
mod field;
mod elf;
mod pipeloop;

use crate::field::Field;
use crate::pipeloop::PipeLoop;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").expect("Can't read input");
    let f = Field::parse(input.as_str(),'S');

    let t = std::time::Instant::now();
    let mut elf = f.get_walking_elf(None);

    let dirs = elf.valid_directions();
    println!("Available directions {:?}",dirs);
    elf.dir = if dirs.is_empty() { panic!("Ops! cannot find valid direction to go!") } else { dirs[0] };

    let mut path = elf.traverse_pipes('S');
    println!("Part 1 : Total steps: {}, furthest away: {} - {:?}", path.len(), path.len()/2, t.elapsed());

    // pre-allocated memory buffer to process each line, so we avoid repeated heap allocations
    let mut pairs: Vec<_> = Vec::with_capacity(20);

    let t = std::time::Instant::now();
    let tiles = path
        // As we'll be scanning line by line we need to
        // group all pipes by `y`, hence extracting the odd/even pairs of pipes
        // and hence measure the number of tiles within each valid pair
        .order_by_scan_lines()
        // scan a line at a time for pairs of pipes
        .map(|line|{
            // clear memory for processing the new line
            pairs.clear();
            // clean & collect valid vertical pipes pairs
            PipeLoop::scanline_cleaner(line, &f)
                .collect_into(&mut pairs);
            // pair up vertical pipes remaining
            pairs.chunks(2)
                // measure the distance from each pair
                .map(|pair| {
                    let [(_,a),(_,b)] = pair else { unreachable!() };
                    b.0 - a.0 - 1
                })
                // Sum up the pairs for this line
                .sum::<usize>()
        })
        // Sum up all lines
        .sum::<usize>();

    println!("Part 2 : Total tiles {} - {:?}", tiles, t.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::direction::Direction::{Down, Left, Right, Up};
    static INPUT_PART1: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
    static INPUT_PART2: &str = ".............\n\
                                .S---------7.\n\
                                .|..F-7.F7.|.\n\
                                .|.FJ.|.|L7|.\n\
                                .|FJ..L-J.||.\n\
                                .|L-7...F-J|.\n\
                                .|..|...|..|.\n\
                                .L--J...L--J.\n\
                                .............";
    #[test]
    fn test_count_area() {
        // let input = std::fs::read_to_string("src/bin/day10/sample1.txt").expect("Ops!");
        let f = Field::parse(INPUT_PART2, 'S');
        let mut elf = f.get_walking_elf(None);

        let dirs = elf.valid_directions();
        println!("Available directions {:?}",dirs);
        elf.dir = if dirs.is_empty() { panic!("Ops! cannot find valid direction to go!") } else { dirs[0] };

        let tiles = elf
            .traverse_pipes('S')
            .order_by_scan_lines()
            .inspect(|c| println!("Group: {:?}",c))
            .map(|pipe|{
                PipeLoop::scanline_cleaner(pipe, &f)
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .inspect(|c| print!("Pair: {:?} -> ",c))
                    .map(|pair| {
                        let [(_,a),(_,b)] = pair else { todo!() };
                        b.0 - a.0 - 1
                    })
                    .inspect(|c| println!("Sum: {:?}",c))
                    .sum::<usize>()
            })
            .inspect(|c| println!("Sum: {:?}\n",c))
            .sum::<usize>();

        assert_eq!(10,tiles);
    }
    #[test]
    fn test_left_right() {
        let f = Field::parse(INPUT_PART2, 'S');

        println!("{:?}", f.connects_right_with((1, 1)));
        println!("{:?}", f.connects_left_with((1, 1)));
        println!("{:?}", f.connects_right_with((4, 2)));
        println!("{:?}", f.connects_left_with((4, 2)));
        println!("{:?}", f.connects_right_with((3, 3)));
        println!("{:?}", f.connects_left_with((3, 3)));
        println!("{:?}", f.connects_right_with((2, 5)));
        println!("{:?}", f.connects_left_with((2, 5)));

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
